# Needed for maturin to expose Rust module properly
from .full_page_recommender import *  # noqa: F403

__doc__ = full_page_recommender.__doc__  # noqa: F405
if hasattr(full_page_recommender, "__all__"):  # noqa: F405
    __all__ = full_page_recommender.__all__  # noqa: F405
