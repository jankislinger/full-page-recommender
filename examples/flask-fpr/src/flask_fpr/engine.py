import abc
from pathlib import Path

from full_page_recommender import PyCollection, recommend

from flask_fpr.models import Carousel, Page, Tile

DATA_DIR = Path(__file__).parents[2] / "data"
assert DATA_DIR.is_dir()


NUM_TILES = 24
NUM_ROWS = 20


class BaseEngine(abc.ABC):
    @abc.abstractmethod
    def get_tile(self, i: int) -> Tile:
        pass

    @abc.abstractmethod
    def get_carousel(self, index: int, content_indices: list[int]) -> Carousel:
        pass

    @abc.abstractmethod
    def score_items(self, user_id: str) -> list[float]:
        pass

    @abc.abstractmethod
    def get_collections(self, user_id: str) -> list[tuple[list[int], float]]:
        pass


def build_page(engine: BaseEngine, user_id: str = "dummy") -> Page:
    scores = engine.score_items(user_id)
    source_collections = engine.get_collections(user_id)

    collections: list[PyCollection] = []
    for item_ids, score in source_collections:
        c = PyCollection([score * scores[i] for i in item_ids], item_ids, is_sorted=False)
        collections.append(c)

    recommendations = recommend(collections, pos_mask(NUM_TILES), num_rows=NUM_ROWS)
    carousels = [engine.get_carousel(i, item_ids) for i, item_ids in recommendations]
    return Page(carousels=carousels)


def pos_mask(n: int, *, offset: int = 1) -> list[float]:
    xs = [1 / i for i in range(offset, offset + n)]
    s = sum(xs)
    return [x / s for x in xs]
