use std::cmp::Ordering;

const TEMP_PENALTY: f64 = 0.3;
const COOLING_FACTOR: f64 = 0.75;
const NUM_ITEMS_ROW: usize = 12;

pub struct Collection {
    scores: Vec<f64>,
    items: Vec<usize>,
    is_sorted: bool,
    is_available: bool,
}

impl Collection {
    fn new(scores: Vec<f64>, items: Vec<usize>, is_sorted: bool) -> Self {
        Self {
            scores,
            items,
            is_sorted,
            is_available: true,
        }
    }
}

pub struct RecommenderState {
    collections: Vec<Collection>,
    item_temps: Vec<f64>,
}

impl RecommenderState {
    pub fn new(
        item_scores: Vec<Vec<f64>>,
        items_in_collections: Vec<Vec<usize>>,
        is_sorted: Vec<bool>,
    ) -> Self {
        // TODO: get rid of `.clone()`; or, better, initialize with collections
        let num_items = guess_num_items(&items_in_collections);
        let collections = item_scores
            .iter()
            .zip(items_in_collections.iter())
            .zip(is_sorted.iter())
            .map(|((scores, items), &sorted)| {
                Collection::new(scores.clone(), items.clone(), sorted)
            })
            .collect();
        Self {
            collections,
            item_temps: vec![0.0; num_items],
        }
    }

    pub fn recommend_page(&mut self, num_rows: usize) -> (Vec<usize>, Vec<Vec<usize>>) {
        let mut collections = Vec::with_capacity(num_rows);
        let mut items_list = Vec::with_capacity(num_rows);
        for _ in 0..num_rows {
            let (collection, items) = self.emit_recommendation();
            collections.push(collection);
            items_list.push(items);
        }
        (collections, items_list)
    }

    fn emit_recommendation(&mut self) -> (usize, Vec<usize>) {
        let (collection, items) = self.recommend_row();
        self.mark_recommendations(collection, &items);
        (collection, items)
    }

    fn recommend_row(&self) -> (usize, Vec<usize>) {
        let item_scores = final_scores(&self.collections, &self.item_temps);
        let collection_idx = find_best_collection(&self.collections, NUM_ITEMS_ROW)
            .expect("no more collections to recommend");

        let items = item_indices(
            &item_scores[collection_idx],
            &self.collections[collection_idx].items,
            NUM_ITEMS_ROW,
        );
        (collection_idx, items)
    }

    fn mark_recommendations(&mut self, collection_idx: usize, items: &[usize]) {
        self.collections[collection_idx].is_available = false;
        self.item_temps = self.item_temps.iter().map(|t| t * COOLING_FACTOR).collect();
        for &item in items {
            self.item_temps[item] += 1.0;
        }
    }
}

fn final_scores(collections: &[Collection], temperature: &[f64]) -> Vec<Vec<f64>> {
    collections
        .iter()
        .map(|col| final_scores_row(col, temperature))
        .collect()
}

fn final_scores_row(collection: &Collection, temperature: &[f64]) -> Vec<f64> {
    collection
        .scores
        .iter()
        .zip(collection.items.iter())
        .map(|(&score, &i)| score * TEMP_PENALTY.powf(temperature[i]))
        .collect()
}

fn item_indices(scores: &[f64], items: &[usize], top_k: usize) -> Vec<usize> {
    let mut scored_items: Vec<(&f64, &usize)> = scores.iter().zip(items).collect();
    scored_items.sort_by(|&(a, _), &(b, _)| b.partial_cmp(a).unwrap_or(Ordering::Equal));
    scored_items
        .iter()
        .map(|&(_, i)| i)
        .take(top_k)
        .cloned()
        .collect()
}

fn find_best_collection(collections: &[Collection], top_k: usize) -> Option<usize> {
    collections
        .iter()
        .enumerate()
        .filter(|(_, col)| col.is_available)
        .map(|(i, col)| (i, collection_score(&col.scores, col.is_sorted, top_k)))
        .max_by(|&(_, a), &(_, b)| a.partial_cmp(&b).unwrap_or(Ordering::Equal))
        .map(|(i, _)| i)
}

fn collection_score(item_scores: &[f64], is_sorted: bool, top_k: usize) -> f64 {
    let total_score: f64 = if is_sorted {
        item_scores.iter().take(top_k).sum()
    } else {
        let mut sorted_scores = item_scores.to_vec();
        sorted_scores.sort_by(|a, b| b.partial_cmp(a).unwrap_or(Ordering::Equal));
        sorted_scores.iter().take(top_k).sum()
    };
    total_score / top_k as f64
}

fn guess_num_items(item_indices: &[Vec<usize>]) -> usize {
    item_indices
        .iter()
        .flat_map(|x| x.iter())
        .max()
        .map(|x| x + 1)
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recommendations_for_one_row() {
        let state = RecommenderState::new(
            vec![vec![0.1, 0.2], vec![0.5, 0.9, 0.2]],
            vec![vec![0, 1], vec![2, 3, 1]],
            vec![false, false],
        );
        let (collection_idx, items) = state.recommend_row();
        assert_eq!(collection_idx, 1);
        assert_eq!(items, vec![3, 2, 1])
    }

    #[test]
    fn test_item_selection() {
        let items = item_indices(&[0.3, 0.5, 0.1, 0.9], &[3, 5, 8, 13], 2);
        assert_eq!(items, vec![13, 5])
    }
}
