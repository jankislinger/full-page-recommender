use pyo3::prelude::*;

mod collection;
mod recommender_state;

#[pyfunction]
#[pyo3(signature = (item_scores, items_in_collections, is_sorted, num_rows, *, num_items_row=8, temp_penalty=0.1, cooling_factor=0.85))]
fn recommend(
    item_scores: Vec<Vec<f64>>,
    items_in_collections: Vec<Vec<usize>>,
    is_sorted: Vec<bool>,
    num_rows: usize,
    num_items_row: usize,
    temp_penalty: f64,
    cooling_factor: f64,
) -> (Vec<usize>, Vec<Vec<usize>>) {
    let mut recommender_state = recommender_state::RecommenderState::from_scores(
        item_scores,
        items_in_collections,
        is_sorted,
    );
    recommender_state.recommend_page(num_rows, num_items_row, temp_penalty, cooling_factor)
}

#[pymodule]
fn full_page_recommender(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(recommend, m)?)?;

    Ok(())
}
