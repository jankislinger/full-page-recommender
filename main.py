import random
import time

from full_page_recommender import recommend, PyCollection


def main() -> None:
    num_collections = 200
    num_items_in_collection = 100
    num_items = 5000
    num_rows = 50
    num_items_row = 24
    sorted_ratio = 0.1

    random.seed(42)

    position_mask = [0.8 ** i for i in range(num_items_row)]
    position_mask = [x / sum(position_mask) for x in position_mask]

    collections = [
        PyCollection(
            scores=[random.random() for _ in range(num_items_in_collection)],
            items=[random.randrange(num_items) for _ in range(num_items_in_collection)],
            is_sorted=random.random() < sorted_ratio,
        )
        for _ in range(num_collections)
    ]

    start_time = time.time()
    collections, items = recommend(collections, position_mask, num_rows=num_rows)
    elapsed = time.time() - start_time

    for i, (col_idx, row_items) in enumerate(zip(collections, items)):
        print(f"Row {i + 1}: Collection {col_idx}")
        print(f"Items: {row_items}")

    print(f"\nElapsed: {elapsed * 1000:.1f}ms")


if __name__ == "__main__":
    main()
