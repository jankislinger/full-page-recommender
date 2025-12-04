import abc
import random
from collections.abc import Iterable
from pathlib import Path

import polars as pl
from full_page_recommender import PyCollection, recommend

from flask_fpr.models import Carousel, Page, Tile

DATA_DIR = Path(__file__).parents[2] / "data"
assert DATA_DIR.is_dir()


NUM_TILES = 15
NUM_ROWS = 20


class BaseEngine(abc.ABC):
    @abc.abstractmethod
    def get_tile(self, i: int) -> Tile:
        pass

    @abc.abstractmethod
    def score_items(self, user_id: str) -> list[float]:
        pass

    @abc.abstractmethod
    def get_collections(self, user_id: str) -> list[tuple[str, list[int]]]:
        pass


class DummyEngine(BaseEngine):
    def __init__(self):
        collections_dir = DATA_DIR / "collections" / "tmdb_keywords"
        collections = pl.scan_parquet(collections_dir / "collections.parquet")
        self.collections_df = (
            pl.scan_parquet(collections_dir / "items_in_collections.parquet")
            .filter(pl.len().over("collection_id") >= NUM_TILES)
            .group_by("collection_id")
            .agg(item_ids=pl.col("item_id"))
            .join(collections, on="collection_id")
            .rename({"collection_score": "score"})
            .collect()
        )

        items = scan_items().collect()
        tiles_catalog = (Tile(**row) for row in items.iter_rows(named=True))
        self.tiles_catalog = {t.id: t for t in tiles_catalog}
        self._frozen_scores: dict[str, list[float]] = {}

    def get_tile(self, i: int) -> Tile:
        return self.tiles_catalog[i]

    def score_items(self, user_id: str) -> list[float]:
        if user_id not in self._frozen_scores:
            print(max(self.tiles_catalog))
            self._frozen_scores[user_id] = [
                random.random() for _ in range(max(self.tiles_catalog) + 1)
            ]
        return self._frozen_scores[user_id]

    def get_collections(self, user_id: str) -> list[tuple[str, list[int]]]:
        return list(self.yield_collections(user_id))

    def yield_collections(self, user_id: str) -> Iterable[tuple[str, list[int]]]:
        _ = user_id  # not used
        rows_iterable = self.collections_df.select("label", "score", "item_ids").iter_rows()
        for label, _score, item_ids in rows_iterable:
            yield label, item_ids


def build_page(engine: BaseEngine) -> Page:
    user_id = "dummy"
    scores = engine.score_items(user_id)
    collections: list[PyCollection] = []
    labels: list[str] = []
    for label, item_ids in engine.get_collections(user_id):
        item_ids = sorted(i for i in set(item_ids) if i in engine.tiles_catalog)
        if len(item_ids) < NUM_TILES:
            continue
        print(item_ids)
        c = PyCollection([scores[i] for i in item_ids], item_ids, is_sorted=False)
        collections.append(c)
        labels.append(label)
    print("num collections:", len(collections))

    recommendations = recommend(collections, pos_mask(NUM_TILES), num_rows=NUM_ROWS)

    carousels = []
    for i, item_ids in recommendations:
        tiles = [engine.get_tile(j) for j in item_ids]
        c = Carousel(id=f"carousel-{i:03d}", title=capitalize_title(labels[i]), tiles=tiles)
        carousels.append(c)

    return Page(carousels=carousels)


def scan_items() -> pl.LazyFrame:
    items = pl.scan_parquet(DATA_DIR / "dataset" / "items.parquet")
    return (
        items
        # .filter(pl.col("image_url").is_not_null())
        .select(
            id=pl.col("item_id"),
            title="title",
            subtitle=pl.col("genres").list.join(", "),
            image_url=pl.col("image_url").fill_null(""),
        )
    )


def pos_mask(n: int, *, offset: int = 1) -> list[float]:
    xs = [1 / i for i in range(offset, offset + n)]
    s = sum(xs)
    return [x / s for x in xs]


PREPS = {"and", "the", "on", "or"}


def capitalize_title(s: str):
    return " ".join(_cap(x, i > 0) for i, x in enumerate(s.split(" ")))


def _cap(x: str, is_in_middle: bool) -> str:
    if is_in_middle and x in PREPS:
        return x
    return x.capitalize()


if __name__ == "__main__":
    e = DummyEngine()
    build_page(e)
