use crate::collection::Collection;
use std::cmp::Ordering;

pub struct RecommenderState {
    collections: Vec<Collection>,
    item_temps: Vec<f64>,
}

impl RecommenderState {
    pub fn new(collections: Vec<Collection>) -> Self {
        let num_items = guess_num_items(&collections);
        Self {
            collections,
            item_temps: vec![0.0; num_items],
        }
    }

    pub fn recommend_page(
        &mut self,
        num_rows: usize,
        num_items_row: usize,
        temp_penalty: f64,
        cooling_factor: f64,
    ) -> (Vec<usize>, Vec<Vec<usize>>) {
        let mut collections = Vec::with_capacity(num_rows);
        let mut items_list = Vec::with_capacity(num_rows);
        for _ in 0..num_rows {
            let (collection, items) =
                self.emit_recommendation(num_items_row, temp_penalty, cooling_factor);
            collections.push(collection);
            items_list.push(items);
        }
        (collections, items_list)
    }

    fn emit_recommendation(
        &mut self,
        num_items: usize,
        temp_penalty: f64,
        cooling_factor: f64,
    ) -> (usize, Vec<usize>) {
        let (collection, items) = self.recommend_row(num_items, temp_penalty);
        self.mark_recommendations(collection, &items, cooling_factor);
        (collection, items)
    }

    fn recommend_row(&self, num_items: usize, temp_penalty: f64) -> (usize, Vec<usize>) {
        let collection_idx =
            find_best_collection(&self.collections, &self.item_temps, num_items, temp_penalty)
                .expect("no more collections to recommend");

        let collection = &self.collections[collection_idx];
        let items = collection.recommend_indices(&self.item_temps, num_items, temp_penalty);
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
}

fn find_best_collection(
    collections: &[Collection],
    item_temps: &[f64],
    top_k: usize,
    temp_penalty: f64,
) -> Option<usize> {
    collections
        .iter()
        .enumerate()
        .filter(|(_, col)| col.is_available)
        .map(|(i, col)| (i, col.score(item_temps, top_k, temp_penalty)))
        .max_by(|&(_, a), &(_, b)| a.partial_cmp(&b).unwrap_or(Ordering::Equal))
        .map(|(i, _)| i)
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
        let state = RecommenderState::new(vec![
            Collection::new(vec![0.1, 0.2], vec![0, 1], false),
            Collection::new(vec![0.5, 0.9, 0.2], vec![2, 3, 1], false),
        ]);
        let (collection_idx, items) = state.recommend_row(3, 0.1);
        assert_eq!(collection_idx, 1);
        assert_eq!(items, vec![3, 2, 1])
    }

    #[test]
    fn recommend_single_sorted_collection() {
        let coll_items = vec![0, 1, 2];
        let state = RecommenderState::new(vec![Collection::new(
            vec![0.1, 0.9, 0.4],
            coll_items.clone(),
            true,
        )]);
        let (_, recom_items) = state.recommend_row(3, 0.1);
        assert_eq!(recom_items, coll_items)
    }

    #[test]
    fn recommend_page_with_deduplication() {
        let mut state = RecommenderState::new(vec![
            Collection::new(vec![0.92, 0.91, 0.90], vec![0, 1, 2], false),
            Collection::new(vec![0.35, 0.31, 0.30], vec![0, 3, 4], false),
            Collection::new(vec![0.32, 0.31, 0.30], vec![5, 6, 7], false),
        ]);
        let (collection_indices, items) = state.recommend_page(3, 3, 0.1, 0.85);
        assert_eq!(collection_indices, vec![0, 2, 1]);
        assert_eq!(items, vec![vec![0, 1, 2], vec![5, 6, 7], vec![3, 4, 0],]);
    }
}
