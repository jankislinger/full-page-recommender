from typing import List, Tuple
import numpy as np

__all__ = ["PyCollection", "PyEaseFPR", "recommend"]

class PyCollection:
    def __init__(
        self, scores: List[float], items: List[int], is_sorted: bool = False
    ) -> None: ...

class PyEaseFPR:
    def __init__(
        self,
        ease_mat: np.ndarray | List[List[float]],
        items_in_collections: List[List[int]],
        position_mask: List[float],
        *,
        num_rows: int,
        temp_penalty: float,
        cooling_factor: float,
    ) -> None: ...
    def recommend(self, history: List[int]) -> List[Tuple[int, List[int]]]: ...

def recommend(
    collections: List[PyCollection],
    position_mask: List[float],
    num_rows: int,
    *,
    temp_penalty: float = 0.1,
    cooling_factor: float = 0.85,
) -> List[Tuple[int, List[int]]]: ...
