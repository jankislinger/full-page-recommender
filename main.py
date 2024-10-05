import random
import time

from full_page_recommender import full_page_recommender

if __name__ == '__main__':
    num_collections = 200
    num_items_in_collection = 100
    num_items = 5000

    random.seed(42)

    item_scores = [
        [random.random() for _ in range(num_items_in_collection)]
        for _ in range(num_collections)
    ]
    items_in_collection = [
        [random.randrange(num_items) for _ in range(num_items_in_collection)]
        for _ in range(num_collections)
    ]
    start_time = time.time()
    collections, items = full_page_recommender.recommend(
        item_scores,
        items_in_collection,
        num_rows=30,
    )
    elapsed = time.time() - start_time

    for i, (col_idx, row_items) in enumerate(zip(collections, items)):
        print(f"Row {i + 1}: Collection {col_idx}")
        print(f"Items: {row_items}")

    print(f"\nElapsed: {elapsed * 1000:.1f}ms")
