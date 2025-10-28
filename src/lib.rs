use crate::collection::Collection;
use crate::ease::EaseRanker;
use crate::ranker::{CollectionDefinition, Ranker};
use crate::recommender_state::RecommenderState;
use pyo3::prelude::*;
use std::sync::Arc;

mod collection;
pub mod ease;
pub mod ranker;
mod recommender_state;

#[pyclass]
struct PyCollection {
    inner: Arc<Collection>,
}

#[pymethods]
impl PyCollection {
    #[new]
    #[pyo3(signature = (scores, items, is_sorted=false))]
    fn py_new(scores: Vec<f64>, items: Vec<usize>, is_sorted: bool) -> Self {
        let inner = Arc::new(Collection::new(&scores, &items, is_sorted));
        PyCollection { inner }
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
    let collections: Vec<Collection> = collections.iter().map(|c| (*c.inner).clone()).collect();
    let mut recommender_state = RecommenderState::new(collections, position_mask);
    recommender_state.recommend_page(num_rows, temp_penalty, cooling_factor)
}

pub struct EaseFPR {
    ranker: EaseRanker,
    collections: Vec<CollectionDefinition>,
    position_mask: Vec<f64>,
    num_rows: usize,
    temp_penalty: f64,
    cooling_factor: f64,
}

impl EaseFPR {
    pub fn new(
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
            .map(|items| CollectionDefinition::new(items, false))
            .collect();
        Self {
            ranker,
            collections,
            position_mask,
            num_rows,
            temp_penalty,
            cooling_factor,
        }
    }

    pub fn recommend(&self, history: &[usize]) -> Vec<(usize, Vec<usize>)> {
        self.ranker.recommend_page(
            history,
            &self.collections,
            &self.position_mask,
            self.num_rows,
            self.temp_penalty,
            self.cooling_factor,
        )
    }
}

#[pyclass]
struct PyEaseFPR {
    inner: Arc<EaseFPR>,
}

#[pymethods]
impl PyEaseFPR {
    #[new]
    #[pyo3(signature = (ease_mat, items_in_collections, position_mask, *, num_rows, temp_penalty, cooling_factor)
    )]
    fn py_new(
        ease_mat: Vec<Vec<f64>>,
        items_in_collections: Vec<Vec<usize>>,
        position_mask: Vec<f64>,
        num_rows: usize,
        temp_penalty: f64,
        cooling_factor: f64,
    ) -> Self {
        let inner = EaseFPR::new(
            ease_mat,
            items_in_collections,
            position_mask,
            num_rows,
            temp_penalty,
            cooling_factor,
        );
        Self {
            inner: Arc::new(inner),
        }
    }

    fn recommend(&self, history: Vec<usize>) -> Vec<(usize, Vec<usize>)> {
        self.inner.recommend(&history)
    }
}

#[pymodule]
mod full_page_recommender {
    #[pymodule_export]
    use super::{PyCollection, PyEaseFPR};

    #[pymodule_export]
    use super::{recommend};
}
