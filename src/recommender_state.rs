use crate::collection::Collection;
use std::cmp::Ordering;

pub struct RecommenderState {
    collections: Vec<Collection>,
    max_scores: Vec<f64>,
    collection_indices: Vec<usize>,
    position_mask: Vec<f64>,
    item_temps: Vec<f64>,
}

impl RecommenderState {
    pub fn new(collections: Vec<Collection>, position_mask: Vec<f64>) -> Self {
        let num_items = guess_num_items(&collections);
        let item_temps = vec![0.0; num_items];
        let mut scored_collections: Vec<(usize, f64, Collection)> = collections
            .into_iter()
            .enumerate()
            .map(|(i, c)| (i, c.score(&item_temps, &position_mask, 0.0), c))
            .collect();
        scored_collections
            .sort_by(|(_, a, _), (_, b, _)| b.partial_cmp(a).unwrap_or(Ordering::Equal));
        let collection_indices = scored_collections.iter().map(|&(i, _, _)| i).collect();
        let max_scores = scored_collections.iter().map(|&(_, s, _)| s).collect();

        let collections = scored_collections
            .into_iter()
            .map(|(_, _, coll)| coll)
            .collect();
        Self {
            collections,
            max_scores,
            collection_indices,
            position_mask,
            item_temps,
        }
    }

    pub fn recommend_page(
        &mut self,
        num_rows: usize,
        temp_penalty: f64,
        cooling_factor: f64,
    ) -> (Vec<usize>, Vec<Vec<usize>>) {
        let mut collections = Vec::with_capacity(num_rows);
        let mut items_list = Vec::with_capacity(num_rows);
        for _ in 0..num_rows {
            let (collection, items) = self.emit_recommendation(temp_penalty, cooling_factor);
            collections.push(collection);
            items_list.push(items);
        }
        (collections, items_list)
    }

    fn emit_recommendation(
        &mut self,
        temp_penalty: f64,
        cooling_factor: f64,
    ) -> (usize, Vec<usize>) {
        // returns position of the original vector
        let (collection, items) = self.recommend_row_impl(temp_penalty);
        self.mark_recommendations(collection, &items, cooling_factor);
        (self.collection_indices[collection], items)
    }

    fn recommend_row_impl(&self, temp_penalty: f64) -> (usize, Vec<usize>) {
        // returns position of the sorted vector
        let collection_idx = self
            .find_best_collection(temp_penalty)
            .expect("no more collections to recommend");

        let collection = &self.collections[collection_idx];
        let items =
            collection.recommend_indices(&self.item_temps, self.position_mask.len(), temp_penalty);
        (collection_idx, items)
    }

    fn mark_recommendations(
        &mut self,
        collection_idx: usize,
        items: &[usize],
        cooling_factor: f64,
    ) {
        self.collections[collection_idx].is_available = false;
        self.item_temps = self.item_temps.iter().map(|t| t * cooling_factor).collect();
        for &item in items {
            self.item_temps[item] += 1.0;
        }
    }

    fn find_best_collection(&self, temp_penalty: f64) -> Option<usize> {
        let mut best: Option<(usize, f64)> = None;
        for (i, coll) in self.collections.iter().enumerate() {
            if !coll.is_available {
                continue;
            }

            if let Some((_, best_val)) = best {
                if best_val >= self.max_scores[i] {
                    break;
                }
            }

            let score = coll.score(&self.item_temps, &self.position_mask, temp_penalty);
            if let Some((_, best_val)) = best {
                if score > best_val {
                    best = Some((i, score));
                }
            } else {
                best = Some((i, score));
            }
        }
        best.map(|(i, _)| i)
    }
}

fn guess_num_items(collections: &[Collection]) -> usize {
    collections
        .iter()
        .flat_map(|x| x.items.iter())
        .max()
        .map(|x| x + 1)
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recommendations_for_one_row() {
        let mut state = RecommenderState::new(
            vec![
                Collection::new(vec![0.1, 0.2], vec![0, 1], false),
                Collection::new(vec![0.5, 0.9, 0.2], vec![2, 3, 1], false),
            ],
            vec![0.6, 0.3, 0.1],
        );
        let (collection_idx, items) = state.emit_recommendation(0.1, 0.0);
        assert_eq!(collection_idx, 1);
        assert_eq!(items, vec![3, 2, 1])
    }

    #[test]
    fn recommend_single_sorted_collection() {
        let coll_items = vec![0, 1, 2];
        let mut state = RecommenderState::new(
            vec![Collection::new(
                vec![0.1, 0.9, 0.4],
                coll_items.clone(),
                true,
            )],
            vec![0.6, 0.3, 0.1],
        );
        let (_, recom_items) = state.emit_recommendation(0.1, 0.0);
        assert_eq!(recom_items, coll_items)
    }

    #[test]
    fn recommend_page_with_deduplication() {
        let mut state = RecommenderState::new(
            vec![
                Collection::new(vec![0.92, 0.91, 0.90], vec![0, 1, 2], false),
                Collection::new(vec![0.35, 0.31, 0.30], vec![0, 3, 4], false),
                Collection::new(vec![0.32, 0.31, 0.30], vec![5, 6, 7], false),
            ],
            vec![0.6, 0.3, 0.1],
        );
        let (collection_indices, items) = state.recommend_page(3, 0.1, 0.85);
        assert_eq!(collection_indices, vec![0, 2, 1]);
        assert_eq!(items, vec![vec![0, 1, 2], vec![5, 6, 7], vec![3, 4, 0],]);
    }
}
