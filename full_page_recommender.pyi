def recommend(
    item_scores: list[list[float]],
    items_in_collections: list[list[int]],
    is_sorted: list[bool],
    num_rows: int,
    num_items_row: int = 8,
    temp_penalty: float = 0.1,
    cooling_factor: float = 0.85,
) -> tuple[list[int], list[list[int]]]: ...
