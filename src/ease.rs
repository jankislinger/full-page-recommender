use crate::ranker::{CollectionDefinition,Ranker};

pub struct EaseRanker {
    mat: Vec<Vec<f64>>,
    num_cols: usize,
}

impl EaseRanker {
    pub fn new(mat: Vec<Vec<f64>>) -> Self {
        let num_cols = mat.first().map(Vec::len).unwrap_or(0);
        Self { mat, num_cols }
    }
}

impl Ranker for EaseRanker {
    type Input = Vec<usize>;

    fn rank_items(&self, inputs: &Vec<usize>) -> Vec<f64> {
        let mut sums = vec![0.0; self.num_cols];
        for &i in inputs {
            for (j, &val) in self.mat[i].iter().enumerate() {
                sums[j] += val;
            }
        }
        for &i in inputs {
            sums[i] = -1e9;
        }
        sums
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ease() {
        let ease = EaseRanker::new(vec![
            vec![0.0, 0.2, 0.9, 0.1],
            vec![0.2, 0.0, 0.2, 0.1],
            vec![0.3, 0.2, 0.0, 0.1],
            vec![0.1, 0.2, 0.3, 0.0],
        ]);
        let inputs = vec![0];
        let collections = vec![
            CollectionDefinition::new(vec![0, 1], false),
            CollectionDefinition::new(vec![1, 2], false),
            CollectionDefinition::new(vec![1, 3], false),
            CollectionDefinition::new(vec![2, 3], false),
        ];
        let position_mask = vec![0.8, 0.2];

        let recommendations =
            ease.recommend_page(&inputs, &collections, &position_mask, 2, 1.0, 1.0);

        assert_eq!(recommendations.len(), 2);
        assert_eq!(recommendations[0], (1, vec![2, 1]));
        assert_eq!(recommendations[1], (3, vec![2, 3]));
    }
}
