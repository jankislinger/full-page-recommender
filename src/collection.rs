use std::cmp::Ordering;

//TODO: avoid pub(crate) attributes
#[derive(Clone)]
pub struct Collection {
    pub(crate) scores: Vec<f64>,
    pub(crate) items: Vec<usize>,
    is_sorted: bool,
    pub(crate) is_available: bool,
}

impl Collection {
    pub fn new(scores: Vec<f64>, items: Vec<usize>, is_sorted: bool) -> Self {
        Self {
            scores,
            items,
            is_sorted,
            is_available: true,
        }
    }

    pub(crate) fn score(&self, item_temps: &[f64], top_k: usize, temp_penalty: f64) -> f64 {
        let item_scores = dedupe_scores(&self.scores, &self.items, item_temps, temp_penalty);
        let total_score: f64 = if self.is_sorted {
            item_scores.iter().take(top_k).sum()
        } else {
            let mut sorted_scores = item_scores.to_vec();
            sorted_scores.sort_by(|a, b| b.partial_cmp(a).unwrap_or(Ordering::Equal));
            sorted_scores.iter().take(top_k).sum()
        };
        total_score / top_k as f64
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
        item_scores.sort_by(|(_, a), (_, b)| b.partial_cmp(a).unwrap_or(Ordering::Equal));
        item_scores
            .iter()
            .take(top_k)
            .map(|&(i, _)| self.items[i])
            .collect()
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
        let col = Collection::new(vec![0.3, 0.5, 0.1, 0.9], vec![3, 5, 8, 13], false);
        let items = col.recommend_indices(&[0.0; 14], 2, 0.5);
        assert_eq!(items, vec![13, 5])
    }
}
