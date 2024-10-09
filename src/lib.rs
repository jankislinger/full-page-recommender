use crate::collection::Collection;
use crate::recommender_state::RecommenderState;
use pyo3::prelude::*;

mod collection;
mod recommender_state;

#[pyclass]
struct PyCollection {
    collection: Collection,
}

#[pymethods]
impl PyCollection {
    #[new]
    #[pyo3(signature = (scores, items, is_sorted=false))]
    fn py_new(scores: Vec<f64>, items: Vec<usize>, is_sorted: bool) -> Self {
        let collection = Collection::new(scores, items, is_sorted);
        PyCollection { collection }
    }
}

#[pyfunction]
#[pyo3(signature = (collections, num_rows, *, num_items_row=8, temp_penalty=0.1, cooling_factor=0.85))]
fn recommend(
    collections: Vec<PyRef<PyCollection>>,
    num_rows: usize,
    num_items_row: usize,
    temp_penalty: f64,
    cooling_factor: f64,
) -> (Vec<usize>, Vec<Vec<usize>>) {
    // TODO: do it without cloning Collection
    let collections: Vec<Collection> = collections
        .into_iter()
        .map(|c| c.collection.clone())
        .collect();
    let mut recommender_state = RecommenderState::new(collections);
    recommender_state.recommend_page(num_rows, num_items_row, temp_penalty, cooling_factor)
}

#[pymodule]
fn full_page_recommender(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyCollection>()?;

    m.add_function(wrap_pyfunction!(recommend, m)?)?;

    Ok(())
}
