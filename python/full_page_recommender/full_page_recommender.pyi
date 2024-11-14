from typing import List, Tuple

class PyCollection:
    def __init__(
        self, index: int, scores: List[float], items: List[int], is_sorted: bool = False
    ) -> None: ...

def recommend(
    collections: List[PyCollection],
    position_mask: List[float],
    num_rows: int,
    temp_penalty: float = 0.1,
    cooling_factor: float = 0.85,
) -> List[Tuple[int, List[int]]]: ...
