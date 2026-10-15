import math
import os
from collections.abc import Callable, Iterable
from functools import lru_cache
from pathlib import Path

import numpy as np
import polars as pl

from flask_fpr.engine import NUM_TILES, BaseEngine
from flask_fpr.models import Carousel, Tile

DATA_DIR = Path(__file__).parents[2] / "data"
assert DATA_DIR.is_dir()

MOVIELENS_ROOT = Path("/home/jkr17/github/phd/clustering") / "data" / "movielens"
EASE_PATH = MOVIELENS_ROOT / "models" / "ease.npy"
LIFT_PATH = MOVIELENS_ROOT / "models" / "lift.npy"
ITEMS_PATH = MOVIELENS_ROOT / "dataset" / "items.parquet"
INTERACTIONS_PATH = MOVIELENS_ROOT / "generated" / "interactions.parquet"

DEV = os.environ.get("DEVELOPMENT", "f") == "true"

BYW_SCORE = 0.95 / math.log(NUM_TILES)


# 90% of what collection with 50 items would get


class EaseEngine(BaseEngine):
    def __init__(self):
        self.ease_mat = np.load(EASE_PATH)
        self.lift_mat = np.load(LIFT_PATH)
        self.items = pl.read_parquet(ITEMS_PATH)
        self.interactions = pl.read_parquet(INTERACTIONS_PATH)

        self.collections = list(_generate_topics(self.items))

        item_tiles = self.items.select(
            id=pl.col("item_id"),
            title=pl.col("title_tmdb"),
            subtitle=pl.col("genres_tmdb").list.join(", "),
            image_url=pl.col("image_url"),
        )

        self.tiles = [tile_from_row(i, row) for i, row in enumerate(item_tiles.iter_rows(named=True))]
        print(self.tiles[:3])

    def get_tile(self, index: int) -> Tile:
        return self.tiles[index]

    def get_carousel(self, index: int, content_indices: list[int]) -> Carousel:
        if index < len(self.collections):
            return Carousel(
                id=f"carousel-{index:03d}",
                title=self.collections[index][0],
                tiles=[self.get_tile(i) for i in content_indices],
            )

        seed_index = index - len(self.collections)
        seed_title = self.tiles[seed_index].title

        return Carousel(
            id=f"carousel-{index:03d}",
            title=f"Because You Watched {seed_title}",
            tiles=[self.get_tile(i) for i in content_indices],
        )

    def score_items(self, user_id: int) -> list[float]:
        watched_indices = self._watched_indices(user_id)
        scores = np.sum(self.ease_mat[watched_indices, :], axis=0)
        scores[watched_indices] = 0.0

        if DEV:
            print("\nBest items:")
            for i in np.argsort(-scores)[:3]:
                tile = self.get_tile(i)
                print(f"id={tile.id:<7} {tile.title}")
            print()

        return list(scores)

    def get_collections(self, user_id: int) -> list[tuple[list[int], float]]:
        common = [(item_ids, score) for label, item_ids, score in self.collections]
        specific = []
        for idx in self._watched_indices(user_id):
            scores = self.lift_mat[idx]
            recom_indices = np.argsort(-scores)[:NUM_TILES].tolist()
            specific.append((recom_indices, BYW_SCORE))
        return common + specific

    def get_collections_2(self, user_id: int) -> list[tuple[list[int], float, list[float]]]:
        common = [(item_ids, score, [1.0] * len(item_ids)) for label, item_ids, score in self.collections]
        specific = []
        byw_affinities = [NUM_TILES/i for i in range(NUM_TILES, 2*NUM_TILES)]
        for idx in self._watched_indices(user_id):
            scores = self.lift_mat[idx]
            recom_indices = np.argsort(-scores)[:NUM_TILES].tolist()
            specific.append((recom_indices, BYW_SCORE, byw_affinities))
        return common + specific

    @lru_cache
    def _watched_indices(self, user_id: int) -> np.ndarray:
        interactions = self.interactions.filter(pl.col("user_id") == int(user_id)).join(
            self.items, on="item_id"
        )
        return interactions["index"].to_numpy()


def tile_from_row(i: int, row: dict) -> Tile:
    return Tile(
        **row,
        badge=str(i) if DEV else None,
    )


PREPS = {"and", "the", "on", "or"}


def capitalize_title(s: str) -> str:
    return " ".join(_cap(x, i > 0) for i, x in enumerate(s.split(" ")))


def _cap(x: str, is_in_middle: bool) -> str:
    if is_in_middle and x in PREPS:
        return x
    if x.startswith("(") and x.endswith(")") and len(x) <= 6:
        return x.upper()
    return x.capitalize()


def _generate_topics(items: pl.DataFrame) -> Iterable[tuple[str, list[int], float]]:
    yield from _inverse_exploded_frame(items, "genres_tmdb").iter_rows(named=False)
    yield from _inverse_exploded_frame(items, "keywords", label_fun=capitalize_title).iter_rows(
        named=False
    )
    yield from _inverse_exploded_frame(
        items, "cast", score_mul=0.9, label_fun=lambda x: f"Starring {x}"
    ).iter_rows(named=False)
    yield from _inverse_exploded_frame(
        items, "directors", score_mul=0.8, label_fun=lambda x: f"Directed by {x}"
    ).iter_rows(named=False)


def _inverse_exploded_frame(
    items: pl.DataFrame,
    col_name: str,
    *,
    score_mul: float = 1.0,
    label_fun: Callable[[str], str] | None = None,
) -> pl.DataFrame:
    num_items = pl.col("item_indices").list.len()
    labels = (
        items.lazy()
        .select("index", col_name)
        .explode(col_name)
        .filter(pl.col(col_name).is_not_null())
        .group_by(col_name)
        .agg(item_indices=pl.col("index").sort())
        .filter(num_items >= NUM_TILES)
        .with_columns(score=num_items.log().pow(-1).mul(score_mul))
    )
    if label_fun is not None:
        labels = labels.with_columns(
            pl.col(col_name).map_elements(label_fun, return_dtype=pl.String)
        )
    return labels.collect()


if __name__ == "__main__":
    from flask_fpr.engine import NUM_TILES, BaseEngine, build_page

    e = EaseEngine()
    build_page(e, "aaa")
