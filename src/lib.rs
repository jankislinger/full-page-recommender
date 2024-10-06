use pyo3::prelude::*;

mod collections;

#[pyfunction]
fn recommend(
    item_scores: Vec<Vec<f64>>,
    items_in_collections: Vec<Vec<usize>>,
    num_rows: usize,
) -> (Vec<usize>, Vec<Vec<usize>>) {
    let num_collections = item_scores.len();
    let mut collections = collections::Collections::new(
        item_scores,
        items_in_collections,
        vec![false; num_collections],
    );
    collections.recommend_page(num_rows)
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn full_page_recommender(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(recommend, m)?)?;

    Ok(())
}