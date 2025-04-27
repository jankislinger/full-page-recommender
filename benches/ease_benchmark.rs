#![feature(test)]

extern crate test;

use full_page_recommender::EaseFPR;
use rand::Rng;
use test::Bencher;
use rand::prelude::IndexedRandom;

fn random_indices(num_items: usize, k_min: usize, k_max: usize) -> Vec<usize> {
    let mut rng = rand::rng();
    let k = rng.random_range(k_min..=k_max);
    (0..num_items).collect::<Vec<_>>()
        .choose_multiple(&mut rng, k)
        .cloned()
        .collect()
}

#[bench]
fn bench_ease_fpr(b: &mut Bencher) {
    let num_items = 10_000;
    let num_collections = 3_000;
    let min_items_per_collection = 50;
    let max_items_per_collection = 300;
    let num_rows = 50;

    // Generate random ease matrix
    let ease_mat: Vec<Vec<f64>> = (0..num_items)
        .map(|_| {
            (0..num_items)
                .map(|_| rand::random::<f64>())
                .collect()
        })
        .collect();

    // Generate random collections
    let items_in_collections: Vec<Vec<usize>> = (0..num_collections)
        .map(|_| random_indices(num_items, min_items_per_collection, max_items_per_collection))
        .collect();

    let position_mask = vec![0.8, 0.2];
    let fpr = EaseFPR::new(
        ease_mat,
        items_in_collections,
        position_mask,
        num_rows,
        0.7, // temp_penalty
        0.2, // cooling_factor
    );

    let history = vec![0];

    b.iter(|| {
        fpr.recommend(&history);
    });
}