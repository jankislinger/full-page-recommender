import random
from pathlib import Path

import polars as pl

from flask_fpr.engine import NUM_TILES, BaseEngine
from flask_fpr.models import Carousel, Tile

DATA_DIR = Path(__file__).parents[2] / "data"
assert DATA_DIR.is_dir()


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

        self.carousels: list[tuple[str, list[int], float]] = []
        rows_iterable = self.collections_df.select("label", "score", "item_ids").iter_rows()
        for label, score, item_ids in rows_iterable:
            label = capitalize_title(label)
            item_ids = sorted(i for i in set(item_ids) if i in self.tiles_catalog)
            if len(item_ids) < NUM_TILES:
                continue
            self.carousels.append((label, item_ids, score))

        self._frozen_scores: dict[str, list[float]] = {}

    def get_tile(self, index: int) -> Tile:
        return self.tiles_catalog[index]

    def get_carousel(self, index: int, content_indices: list[int]) -> Carousel:
        return Carousel(
            id=f"carousel-{index:03d}",
            title=self.carousels[index][0],
            tiles=[self.get_tile(i) for i in content_indices],
        )

    def score_items(self, user_id: str) -> list[float]:
        if user_id not in self._frozen_scores:
            num_items_cap = max(self.tiles_catalog) + 1
            self._frozen_scores[user_id] = [random.random() for _ in range(num_items_cap)]
        return self._frozen_scores[user_id]

    def get_collections(self, user_id: str) -> list[tuple[list[int], float]]:
        return [(item_ids, score) for label, item_ids, score in self.carousels]


def scan_items() -> pl.LazyFrame:
    return (
        pl.scan_parquet(DATA_DIR / "dataset" / "items.parquet")
        .filter(pl.col("image_url").is_not_null())
        .select(
            id=pl.col("item_id"),
            title="title",
            subtitle=pl.col("genres").list.join(", "),
            image_url=pl.col("image_url"),
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
    from flask_fpr.engine import NUM_TILES, BaseEngine, build_page

    e = DummyEngine()
    build_page(e)
