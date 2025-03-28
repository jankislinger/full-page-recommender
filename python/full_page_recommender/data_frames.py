import polars as pl

from full_page_recommender import PyCollection, recommend


def recommend_frames_2(
    items: pl.DataFrame, collections: pl.DataFrame, items_in_collections: pl.DataFrame
) -> pl.DataFrame:
    """Run recommender from a data frame.

    Args:
        items (pl.DataFrame): A data frame containing items.
        collections (pl.DataFrame): A data frame containing collections.
        items_in_collections (pl.DataFrame): A data frame containing items in collections.

    Returns:
        pl.DataFrame: A data frame containing recommendations.

    Examples:
        >>> items_ = pl.DataFrame({
        ...     "item_id": [0, 1, 2, 3, 4],
        ...     "item_score": [0.5, 0.1, 0.3, 0.3, 0.2],
        ... })
        >>> collections_ = pl.DataFrame({
        ...     "collection_id": [0, 1],
        ...     "is_sorted": [False, True],
        ...     "collection_score": [1.0, 0.3]
        ... })
        >>> items_in_collections_ = pl.DataFrame({
        ...     "collection_id": [0, 0, 1, 1, 1, 1],
        ...     "item_id": [1, 4, 0, 1, 2, 3],
        ...     "affinity": [0.5, 0.1, 0.3, 0.3, 0.2, 0.1],
        ... })
        >>> recommend_frames_2(items_, collections_, items_in_collections_)
        shape: (2, 4)
        ┌─────────┬────────────┬───────────────┬─────────┐
        │ row_idx ┆ column_idx ┆ collection_id ┆ item_id │
        │ ---     ┆ ---        ┆ ---           ┆ ---     │
        │ i64     ┆ i64        ┆ i64           ┆ i64     │
        ╞═════════╪════════════╪═══════════════╪═════════╡
        │ 0       ┆ 0          ┆ 0             ┆ 1       │
        │ 0       ┆ 1          ┆ 0             ┆ 4       │
        └─────────┴────────────┴───────────────┴─────────┘
    """
    collections = (
        items.join(items_in_collections, on="item_id")
        .join(collections, on="collection_id")
        .with_columns(score=score_fun())
        .group_by("collection_id", "is_sorted")
        .agg(
            pl.col("item_id").alias("item_ids"),
            pl.col("score").alias("scores"),
        )
        .sort("collection_id")
        .with_columns(collection_index=pl.int_range(pl.len()))
    )
    py_collections = into_collection_list(collections)
    recommendations = recommend(py_collections, [0.8, 0.2], 1)
    return (
        pl.DataFrame(
            recommendations, schema=["collection_index", "item_ids"], orient="row"
        )
        .with_columns(pl.int_range(pl.len()).alias("row_idx"))
        .join(
            collections.select("collection_id", "collection_index"),
            on="collection_index",
        )
        .drop("collection_index")
        .explode("item_ids")
        .rename({"item_ids": "item_id"})
        .with_columns(pl.int_range(pl.len()).over("row_idx").alias("column_idx"))
        .select("row_idx", "column_idx", "collection_id", "item_id")
        .sort("row_idx", "column_idx")
    )


def score_fun() -> pl.Expr:
    return pl.col("item_score") * pl.col("collection_score") * pl.col("affinity")


def recommend_frame(
    collections: pl.DataFrame, items_in_collections: pl.DataFrame
) -> pl.DataFrame:
    """Run recommender from a data frame.

    Args:
        collections (pl.DataFrame): A data frame containing collections.
        items_in_collections (pl.DataFrame): A data frame containing items in collections.

    Returns:
        pl.DataFrame: A data frame containing recommendations.

    Examples:
        >>> collections_ = pl.DataFrame({
        ...     "index": [0, 1],
        ...     "is_sorted": [False, True],
        ... })
        >>> items_in_collections_ = pl.DataFrame({
        ...     "collection": [0, 0, 1, 1, 1, 1],
        ...     "item": [1, 4, 0, 1, 2, 3],
        ...     "score": [0.5, 0.1, 0.3, 0.3, 0.2, 0.1],
        ... })
        >>> recommend_frame(collections_, items_in_collections_)
        shape: (2, 4)
        ┌────────────┬───────┬─────────────────────┬───────────────┐
        │ collection ┆ items ┆ collection_position ┆ item_position │
        │ ---        ┆ ---   ┆ ---                 ┆ ---           │
        │ i64        ┆ i64   ┆ i64                 ┆ i64           │
        ╞════════════╪═══════╪═════════════════════╪═══════════════╡
        │ 0          ┆ 1     ┆ 0                   ┆ 0             │
        │ 0          ┆ 4     ┆ 0                   ┆ 1             │
        └────────────┴───────┴─────────────────────┴───────────────┘
    """
    collections = (
        items_in_collections.group_by("collection")
        .agg(pl.col("item"), pl.col("score"))
        .join(collections, left_on="collection", right_on="index")
        .sort("collection")
    )
    py_collections = [
        PyCollection(row["collection"], row["score"], row["item"], row["is_sorted"])
        for row in collections.iter_rows(named=True)
    ]
    recommendations = recommend(py_collections, [0.8, 0.2], 1)
    return (
        pl.DataFrame(recommendations, schema=["collection", "items"], orient="row")
        .with_columns(pl.int_range(pl.len()).alias("collection_position"))
        .explode("items")
        .with_columns(pl.int_range(pl.len()).over("collection").alias("item_position"))
    )


def into_collection_list(collections: pl.DataFrame) -> list[PyCollection]:
    collections = collections.select(
        "collection_index", "scores", "item_ids", "is_sorted"
    )
    return [
        PyCollection(
            row["collection_index"], row["scores"], row["item_ids"], row["is_sorted"]
        )
        for row in collections.iter_rows(named=True)
    ]
