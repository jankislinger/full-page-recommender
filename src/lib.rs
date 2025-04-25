use crate::collection::Collection;
use crate::ease::EaseRanker;
use crate::ranker::{CollectionDefinition, Ranker};
use crate::recommender_state::RecommenderState;
use pyo3::prelude::*;

mod collection;
mod ease;
mod ranker;
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
#[pyo3(signature = (collections, position_mask, num_rows, *, temp_penalty=0.1, cooling_factor=0.85))]
fn recommend(
    collections: Vec<PyRef<PyCollection>>,
    position_mask: Vec<f64>,
    num_rows: usize,
    temp_penalty: f64,
    cooling_factor: f64,
) -> Vec<(usize, Vec<usize>)> {
    // TODO: do it without cloning Collection
    let collections: Vec<Collection> = collections.iter().map(|c| c.collection.clone()).collect();
    let mut recommender_state = RecommenderState::new(collections, position_mask);
    recommender_state.recommend_page(num_rows, temp_penalty, cooling_factor)
}

#[pyclass]
struct PyEaseFPR {
    ranker: EaseRanker,
    collections: Vec<CollectionDefinition>,
    position_mask: Vec<f64>,
    num_rows: usize,
    temp_penalty: f64,
    cooling_factor: f64,
}

#[pymethods]
impl PyEaseFPR {
    #[new]
    #[pyo3(signature = (ease_mat, items_in_collections, position_mask, *, num_rows, temp_penalty, cooling_factor))]
    fn py_new(
        ease_mat: Vec<Vec<f64>>,
        items_in_collections: Vec<Vec<usize>>,
        position_mask: Vec<f64>,
        num_rows: usize,
        temp_penalty: f64,
        cooling_factor: f64,
    ) -> Self {
        let ranker = EaseRanker::new(ease_mat);
        let collections = items_in_collections
            .iter()
            .map(|items| CollectionDefinition::new(items.to_vec(), false))
            .collect();
        PyEaseFPR {
            ranker,
            collections,
            position_mask,
            num_rows,
            temp_penalty,
            cooling_factor,
        }
    }

    fn recommend(&self, history: Vec<usize>) -> Vec<(usize, Vec<usize>)> {
        self.ranker.recommend_page(
            &history,
            &self.collections,
            &self.position_mask,
            self.num_rows,
            self.temp_penalty,
            self.cooling_factor,
        )
    }
}

#[pymodule]
fn full_page_recommender(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyCollection>()?;
    m.add_class::<PyEaseFPR>()?;

    m.add_function(wrap_pyfunction!(recommend, m)?)?;

    Ok(())
}
