use core::slice::Iter;
use serde::Deserialize;
use std::cmp::Ordering;
use std::str::FromStr;
use std::sync::Arc;

#[derive(Clone, Debug, Deserialize)]
pub struct Collection {
    scores: Arc<[f64]>,
    items: Arc<[usize]>,
    is_sorted: bool,
}

impl Collection {
    pub fn new(scores: &[f64], items: &[usize], is_sorted: bool) -> Self {
        Self {
            scores: Arc::from(scores),
            items: Arc::from(items),
            is_sorted,
        }
    }

    pub fn iter_items(&self) -> Iter<usize> {
        self.items.iter()
    }

    #[allow(dead_code)]
    fn score_geom(&self, item_temps: &[f64], top_k: usize, q: f64, temp_penalty: f64) -> f64 {
        let tot = (1.0 - q.powi(top_k as i32)) / (1.0 - q);
        let position_mask: Vec<f64> = (0..top_k).map(|i| q.powi(i as i32) / tot).collect();
        self.score(item_temps, &position_mask, temp_penalty)
    }

    pub(crate) fn score(
        &self,
        item_temps: &[f64],
        position_mask: &[f64],
        temp_penalty: f64,
    ) -> f64 {
        let mut item_scores = dedupe_scores(&self.scores, &self.items, item_temps, temp_penalty);
        if !self.is_sorted {
            item_scores.sort_by(rev_cmp_float)
        }
        item_scores
            .iter()
            .zip(position_mask.iter())
            .map(|(&a, &b)| a * b)
            .sum()
    }

    pub(crate) fn potential(&self, position_mask: &[f64]) -> f64 {
        let mut item_scores = self.scores.to_vec();
        if !self.is_sorted {
            item_scores.sort_by(rev_cmp_float)
        }
        item_scores
            .iter()
            .zip(position_mask.iter())
            .map(|(&a, &b)| a * b)
            .sum()
    }

    pub(crate) fn recommend_indices(
        &self,
        item_temps: &[f64],
        top_k: usize,
        temp_penalty: f64,
    ) -> Vec<usize> {
        if self.is_sorted {
            return self.items.iter().take(top_k).cloned().collect();
        }

        let mut item_scores: Vec<(usize, f64)> =
            dedupe_scores(&self.scores, &self.items, item_temps, temp_penalty)
                .iter()
                .enumerate()
                .map(|(i, &x)| (i, x))
                .collect();
        item_scores.sort_by(|(_, a), (_, b)| rev_cmp_float(a, b));
        item_scores
            .iter()
            .take(top_k)
            .map(|&(i, _)| self.items[i])
            .collect()
    }
}

impl FromStr for Collection {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}

fn dedupe_scores(
    scores: &[f64],
    items: &[usize],
    item_temps: &[f64],
    temp_penalty: f64,
) -> Vec<f64> {
    scores
        .iter()
        .zip(items.iter())
        .map(|(&score, &i)| score * temp_penalty.powf(item_temps[i]))
        .collect()
}

pub(crate) fn rev_cmp_float(a: &f64, b: &f64) -> Ordering {
    b.partial_cmp(a).expect("Values must be numbers")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_item_selection() {
        let col = Collection::new(&[0.3, 0.5, 0.1, 0.9], &[3, 5, 8, 13], false);
        let items = col.recommend_indices(&[0.0; 14], 2, 0.5);
        assert_eq!(items, vec![13, 5]);
    }

    #[test]
    fn test_parsing_json_valid() {
        let json_str = r#"
            {
                "index": 1,
                "scores": [0.5, 1.0],
                "items": [10, 20],
                "is_sorted": true,
                "is_available": false
            }
        "#;
        json_str
            .parse::<Collection>()
            .expect("Valid string should be parsed");
    }

    #[test]
    fn test_parsing_json_invalid() {
        let json_str = "invalid string";
        json_str
            .parse::<Collection>()
            .expect_err("Invalid string shouldn't be parsed");
    }

    #[test]
    fn test_rev_cmp_float() {
        assert_eq!(rev_cmp_float(&3.0, &1.0), Ordering::Less);
        assert_eq!(rev_cmp_float(&3.0, &5.0), Ordering::Greater);
        assert_eq!(rev_cmp_float(&3.0, &f64::NEG_INFINITY), Ordering::Less);
        assert_eq!(rev_cmp_float(&3.0, &f64::INFINITY), Ordering::Greater);
    }

    #[test]
    #[should_panic(expected = "Values must be numbers")]
    fn test_rev_cmp_float_panic() {
        rev_cmp_float(&f64::NAN, &1.0);
    }
}
