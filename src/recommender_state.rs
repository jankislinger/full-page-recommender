use crate::collection::{rev_cmp_float, Collection};
use std::cmp::Ordering;
use std::sync::Arc;

pub struct RecommenderState {
    collections: Arc<[Collection]>,
    is_available: Vec<bool>,
    max_scores: Vec<f64>,
    position_mask: Vec<f64>,
    indices: Vec<usize>,
    item_temps: Vec<f64>,
}

impl RecommenderState {
    pub fn new(collections: &[Collection], position_mask: Vec<f64>) -> Self {
        let num_items = guess_num_items(collections);
        let mut scored_collections: Vec<(usize, f64, &Collection)> = collections
            .iter()
            .enumerate()
            .map(|(i, c)| (i, c.potential(&position_mask), c))
            .collect();
        scored_collections.sort_by(rev_cmp_second_element);

        let indices = scored_collections.iter().map(|&(i, _, _)| i).collect();
        let max_scores = scored_collections.iter().map(|&(_, s, _)| s).collect();
        let collections: Vec<Collection> = scored_collections
            .into_iter()
            .map(|(_, _, c)| c.clone())
            .collect();
        let num_collections = collections.len();
        let collections = collections.into();

        Self {
            collections,
            is_available: vec![true; num_collections],
            max_scores,
            position_mask,
            indices,
            item_temps: vec![0.0; num_items],
        }
    }

    pub fn recommend_page(
        &mut self,
        num_rows: usize,
        temp_penalty: f64,
        cooling_factor: f64,
    ) -> Vec<(usize, Vec<usize>)> {
        (0..num_rows)
            .map_while(|_| self.emit_recommendation(temp_penalty, cooling_factor))
            .collect()
    }

    fn emit_recommendation(
        &mut self,
        temp_penalty: f64,
        cooling_factor: f64,
    ) -> Option<(usize, Vec<usize>)> {
        let i: usize = self.find_best_collection(temp_penalty)?;
        let coll_idx = self.indices[i];
        let collection = &self.collections.as_ref()[i];
        let items =
            collection.recommend_indices(&self.item_temps, self.position_mask.len(), temp_penalty);
        self.is_available[i] = false;
        self.update_temperatures(&items, cooling_factor);
        Some((coll_idx, items))
    }

    fn update_temperatures(&mut self, items: &[usize], cooling_factor: f64) {
        self.item_temps = self.item_temps.iter().map(|t| t * cooling_factor).collect();
        for &item in items {
            self.item_temps[item] += 1.0;
        }
    }

    fn find_best_collection(&self, temp_penalty: f64) -> Option<usize> {
        let mut best: Option<(usize, f64)> = None;
        for (i, coll) in self.collections.iter().enumerate() {
            if !self.is_available[i] {
                continue;
            }

            if let Some((_, best_val)) = best {
                if best_val >= self.max_scores[i] {
                    break;
                }
            }

            let score = coll.score(&self.item_temps, &self.position_mask, temp_penalty);
            best = match best {
                Some((_, best_val)) if score > best_val => Some((i, score)),
                None => Some((i, score)),
                _ => best,
            };
        }
        best.map(|(i, _)| i)
    }
}

fn guess_num_items(collections: &[Collection]) -> usize {
    collections
        .iter()
        .flat_map(|x| x.iter_items())
        .max()
        .map(|x| x + 1)
        .unwrap_or(0)
}

fn rev_cmp_second_element<T, U>(a: &(T, f64, U), b: &(T, f64, U)) -> Ordering {
    rev_cmp_float(&a.1, &b.1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recommendations_for_one_row() {
        let mut state = RecommenderState::new(
            &vec![
                Collection::new(&[0.1, 0.2], &[0, 1], false),
                Collection::new(&[0.5, 0.9, 0.2], &[2, 3, 1], false),
            ],
            vec![0.6, 0.3, 0.1],
        );
        let (collection_idx, items) = state.emit_recommendation(0.1, 0.0).unwrap();
        assert_eq!(collection_idx, 1);
        assert_eq!(&items, &[3, 2, 1]);
    }

    #[test]
    fn recommend_single_sorted_collection() {
        let coll_items = vec![0, 1, 2];
        let mut state = RecommenderState::new(
            &vec![Collection::new(&[0.1, 0.9, 0.4], &coll_items, true)],
            vec![0.6, 0.3, 0.1],
        );
        let (_, recom_items) = state.emit_recommendation(0.1, 0.0).unwrap();
        assert_eq!(&recom_items, &coll_items);
    }

    #[test]
    fn recommend_page_with_deduplication() {
        let mut state = RecommenderState::new(
            &vec![
                Collection::new(&[0.92, 0.91, 0.90], &[0, 1, 2], false),
                Collection::new(&[0.35, 0.31, 0.30], &[0, 3, 4], false),
                Collection::new(&[0.32, 0.31, 0.30], &[5, 6, 7], false),
            ],
            vec![0.6, 0.3, 0.1],
        );
        let rows = state.recommend_page(3, 0.1, 0.85);
        let expected = vec![(0, vec![0, 1, 2]), (2, vec![5, 6, 7]), (1, vec![3, 4, 0])];
        assert_eq!(&rows, &expected);
    }
}
