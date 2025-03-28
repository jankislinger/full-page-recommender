use core::slice::Iter;
use std::cmp::Ordering;

#[derive(Clone, Debug)]
pub struct Collection {
    index: usize,
    scores: Vec<f64>,
    items: Vec<usize>,
    is_sorted: bool,
    is_available: bool,
}

impl Collection {
    pub fn new(index: usize, scores: Vec<f64>, items: Vec<usize>, is_sorted: bool) -> Self {
        Self {
            index,
            scores,
            items,
            is_sorted,
            is_available: true,
        }
    }

    pub fn iter_items(&self) -> Iter<usize> {
        self.items.iter()
    }

    pub fn disable(&mut self) {
        self.is_available = false;
    }

    pub fn is_available(&self) -> bool {
        self.is_available
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
            item_scores.sort_by(|a, b| b.partial_cmp(a).unwrap_or(Ordering::Equal))
        }
        item_scores
            .iter()
            .zip(position_mask.iter())
            .map(|(&a, &b)| a * b)
            .sum()
    }
    pub(crate) fn potential(&self, position_mask: &[f64]) -> f64 {
        let mut item_scores = self.scores.clone();
        if !self.is_sorted {
            item_scores.sort_by(|a, b| b.partial_cmp(a).unwrap_or(Ordering::Equal))
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
    ) -> (usize, Vec<usize>) {
        if self.is_sorted {
            return (self.index, self.items.iter().take(top_k).cloned().collect());
        }

        let mut item_scores: Vec<(usize, f64)> =
            dedupe_scores(&self.scores, &self.items, item_temps, temp_penalty)
                .iter()
                .enumerate()
                .map(|(i, &x)| (i, x))
                .collect();
        item_scores.sort_by(|(_, a), (_, b)| b.partial_cmp(a).unwrap_or(Ordering::Equal));
        let item_indices: Vec<usize> = item_scores
            .iter()
            .take(top_k)
            .map(|&(i, _)| self.items[i])
            .collect();
        (self.index, item_indices)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_item_selection() {
        let col = Collection::new(0, vec![0.3, 0.5, 0.1, 0.9], vec![3, 5, 8, 13], false);
        let (i, items) = col.recommend_indices(&[0.0; 14], 2, 0.5);
        assert_eq!(i, 0);
        assert_eq!(items, vec![13, 5]);
    }
}
