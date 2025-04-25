import random
import time
import unittest
import numpy as np
from full_page_recommender import PyCollection, recommend, PyEaseFPR


class TestPyCollection(unittest.TestCase):
    def test_py_collections_init(self):
        PyCollection(scores=[0.5, 0.1], items=[1, 4], is_sorted=False)


class TestRecommend(unittest.TestCase):
    def test_recommend(self):
        collections = [
            PyCollection([0.5, 0.1], [1, 4], False),
            PyCollection([0.3, 0.3, 0.2, 0.1], [0, 1, 2, 3], True),
        ]
        recommend(
            collections=collections,
            position_mask=[0.8, 0.2],
            num_rows=1,
        )


class TestEaseFPR(unittest.TestCase):
    def test_recommend(self):
        ease_mat = [
            [0.0, 0.2, 0.9, 0.1],
            [0.2, 0.0, 0.2, 0.1],
            [0.3, 0.2, 0.0, 0.1],
            [0.1, 0.2, 0.3, 0.0],
        ]
        items_in_collections = [
            [0, 1],
            [1, 2],
            [1, 3],
            [2, 3],
        ]
        position_mask = [0.8, 0.2]
        fpr = PyEaseFPR(
            ease_mat,
            items_in_collections,
            position_mask,
            num_rows=2,
            temp_penalty=1.0,
            cooling_factor=1.0,
        )
        recommendations = fpr.recommend(history=[0])
        expected = [(1, [2, 1]), (3, [2, 3])]
        self.assertListEqual(recommendations, expected)

    def test_benchmark(self):
        num_items = 10_000
        num_collections = 3000
        min_items_per_collection = 50
        max_items_per_collection = 300
        num_rows = 50

        ease_mat = np.random.random((num_items, num_items))
        items_in_collections = [
            random_indices(
                num_items, min_items_per_collection, max_items_per_collection
            )
            for _ in range(num_collections)
        ]
        position_mask = [0.8, 0.2]
        fpr = PyEaseFPR(
            ease_mat,
            items_in_collections,
            position_mask,
            num_rows=num_rows,
            temp_penalty=0.7,
            cooling_factor=0.2,
        )
        start = time.time()
        fpr.recommend(history=[0])
        elapsed = time.time() - start
        print(f"{elapsed * 1000:.2f}ms")


def random_indices(num_items: int, k_min: int, k_max: int) -> list[int]:
    k = random.randint(k_min, k_max)
    return random.sample(range(num_items), k=k)


if __name__ == "__main__":
    unittest.main()
