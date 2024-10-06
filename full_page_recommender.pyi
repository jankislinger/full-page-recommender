def recommend(
    item_scores: list[list[float]],
    items_in_collections: list[list[int]],
    is_sorted: list[bool],
    num_rows: int,
) -> tuple[list[int], list[int, int]]: ...
