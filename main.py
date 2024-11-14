import random
import time

from full_page_recommender import PyCollection, recommend


def main() -> None:
    num_collections = 1_000
    num_items_in_collection = 250
    num_items = 10_000
    num_rows = 100
    num_items_row = 32
    sorted_ratio = 0.1

    random.seed(42)

    position_mask = [0.8**i for i in range(num_items_row)]
    position_mask = [x / sum(position_mask) for x in position_mask]

    collections = [
        PyCollection(
            index=i,
            scores=[random.random() for _ in range(num_items_in_collection)],
            items=[random.randrange(num_items) for _ in range(num_items_in_collection)],
            is_sorted=random.random() < sorted_ratio,
        )
        for i in range(num_collections)
    ]

    start_time = time.time()
    rows = recommend(collections, position_mask, num_rows=num_rows)
    elapsed = time.time() - start_time

    for i, (col_idx, row_items) in enumerate(rows):
        print(f"Row {i + 1}: Collection {col_idx}")
        print(f"Items: {row_items}")

    print(f"\nElapsed: {elapsed * 1000:.1f}ms")


if __name__ == "__main__":
    main()
