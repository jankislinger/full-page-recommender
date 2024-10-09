import random
import time

from full_page_recommender import recommend, PyCollection


def main() -> None:
    num_collections = 200
    num_items_in_collection = 100
    num_items = 5000
    sorted_ratio = 0.1

    random.seed(42)


    collections = [
        PyCollection(
            scores=[random.random() for _ in range(num_items_in_collection)],
            items=[random.randrange(num_items) for _ in range(num_items_in_collection)],
            is_sorted=random.random() < sorted_ratio,
        )
        for _ in range(num_collections)
    ]

    start_time = time.time()
    collections, items = recommend(collections, num_rows=30, num_items_row=8)
    elapsed = time.time() - start_time

    for i, (col_idx, row_items) in enumerate(zip(collections, items)):
        print(f"Row {i + 1}: Collection {col_idx}")
        print(f"Items: {row_items}")

    print(f"\nElapsed: {elapsed * 1000:.1f}ms")


if __name__ == "__main__":
    main()
