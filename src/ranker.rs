use crate::collection::Collection;
use crate::recommender_state::RecommenderState;

pub trait Ranker {
    type Input: ?Sized;
    fn rank_items(&self, inputs: &Self::Input) -> Vec<f64>;

    fn recommend_page(
        &self,
        inputs: &Self::Input,
        collections: &[CollectionDefinition],
        position_mask: &[f64],
        num_rows: usize,
        temp_penalty: f64,
        cooling_factor: f64,
    ) -> Vec<(usize, Vec<usize>)> {
        let item_scores = self.rank_items(inputs);
        let collections = score_collections(collections, &item_scores);
        let mut rs = RecommenderState::new(collections, position_mask.into());
        rs.recommend_page(num_rows, temp_penalty, cooling_factor)
    }
}

pub struct CollectionDefinition {
    items: Vec<usize>,
    is_sorted: bool,
}

impl CollectionDefinition {
    pub fn new(items: Vec<usize>, is_sorted: bool) -> Self {
        Self { items, is_sorted }
    }
}

impl CollectionDefinition {
    fn as_collection(&self, scores: &[f64]) -> Collection {
        let scores = self.items.iter().map(|&i| scores[i]).collect();
        Collection::new(scores, self.items.clone(), self.is_sorted)
    }
}

fn score_collections(collections: &[CollectionDefinition], item_scores: &[f64]) -> Vec<Collection> {
    collections
        .iter()
        .map(|c| c.as_collection(item_scores))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct LinearRanker {
        num_items: usize,
    }

    impl Ranker for LinearRanker {
        type Input = ();
        fn rank_items(&self, _: &Self::Input) -> Vec<f64> {
            let n = self.num_items as f64;
            (0..self.num_items).map(|x| x as f64 / n).collect()
        }
    }

    #[test]
    fn test_dummy_ranker() {
        let ranker = LinearRanker { num_items: 10 };
        let collections = vec![
            CollectionDefinition {
                items: vec![0, 1, 2, 3],
                is_sorted: false,
            },
            CollectionDefinition {
                items: vec![5, 6, 7, 8],
                is_sorted: false,
            },
            CollectionDefinition {
                items: vec![7, 8, 9],
                is_sorted: false,
            },
        ];
        let position_mask = vec![0.8, 0.2];
        let recommendations = ranker.recommend_page(&(), &collections, &position_mask, 1, 0.7, 0.7);
        assert_eq!(recommendations.len(), 1);
        assert_eq!(recommendations[0].0, 2);
        assert_eq!(recommendations[0].1, vec![9, 8]);
    }
}
