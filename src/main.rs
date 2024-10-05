use std::time::Instant;
use rand::Rng;

mod collections;

fn main() {
    let num_collections = 200;
    let num_items_in_collection = 100;
    let num_items = 5000;

    let item_scores = random_scores(num_collections, num_items_in_collection);
    let items_in_collections = random_indices(num_collections, num_items_in_collection, num_items);
    let is_sorted = random_vector_bools(num_collections);

    let mut collections = collections::Collections::new(
        item_scores,
        items_in_collections,
        is_sorted,
    );
    let start = Instant::now();
    let (collection_indices, items_list) = collections.recommend_page(30);


    for (i, (&collection_idx, items)) in collection_indices.iter().zip(items_list).enumerate() {
        println!("Row {}: Collection {}", i + 1, collection_idx);
        println!("Items: {:?}", items);
    }
    println!("Elapsed: {:.2?}", start.elapsed());
}

fn random_vector_floats(n: usize) -> Vec<f64> {
    let mut rng = rand::thread_rng();
    (0..n).map(|_| rng.random()).collect()
}

fn random_vector_ints(n: usize, high: usize) -> Vec<usize> {
    let mut rng = rand::thread_rng();
    (0..n).map(|_| rng.gen_range(0..high)).collect()
}

fn random_vector_bools(n: usize) -> Vec<bool> {
    let mut rng = rand::thread_rng();
    (0..n).map(|_| rng.gen_bool(0.1)).collect()
}

fn random_scores(num_rows: usize, num_cols: usize) -> Vec<Vec<f64>> {
    (0..num_rows).map(|_| random_vector_floats(num_cols)).collect()
}
fn random_indices(num_rows: usize, num_cols: usize, high: usize) -> Vec<Vec<usize>> {
    (0..num_rows).map(|_| random_vector_ints(num_cols, high)).collect()
}
