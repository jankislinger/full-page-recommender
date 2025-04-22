import unittest

from full_page_recommender import PyCollection, recommend


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


if __name__ == "__main__":
    unittest.main()
