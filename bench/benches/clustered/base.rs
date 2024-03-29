use crate::utils::{euclidean_squared, Neighbor};
use criterion::Criterion;
use ordered_float::OrderedFloat;
use rand::{rngs::StdRng, seq::SliceRandom, SeedableRng};
use srtree::SRTree;
use std::collections::BinaryHeap;

use super::data::covtype54d_dataset;
#[allow(unused)]
use super::data::{
    audio_dataset, darpa_audio_dataset, dns_dataset, glove100d_dataset, glove50d_dataset,
    home_dataset, world_cities_dataset,
};

const K: usize = 15; // number of nearest neighbors

fn benchmark_dataset() -> Vec<Vec<f64>> {
    let pts = covtype54d_dataset();
    let pts: Vec<Vec<f64>> = pts.into_iter().map(|p| p.to_vec()).collect();
    pts
}

fn query_dataset(n: usize) -> Vec<Vec<f64>> {
    let pts = benchmark_dataset();
    let mut pts: Vec<Vec<f64>> = pts.into_iter().map(|p| p.to_vec()).collect();
    let mut rng = StdRng::from_seed(*b"PiH6Xi3GBBXhTK6UsXJYngHaF3fx4aYS");
    pts.shuffle(&mut rng);
    pts.truncate(n);
    pts
}

fn build(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("build");
    group.sample_size(10);

    let dataset = benchmark_dataset();
    println!("dataset size: {}, dim: {}", dataset.len(), dataset[0].len());

    // benchmark build performance of bulk-loading
    group.bench_function("bulk-loading", |bencher| {
        bencher.iter(|| {
            let pts = benchmark_dataset();
            let pts: Vec<Vec<f64>> = pts.into_iter().map(|p| p.to_vec()).collect();
            SRTree::euclidean(&pts)
        });
    });
}

fn query(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("query");
    group.sample_size(10);

    // query points
    let query_points = query_dataset(1000);

    // benchmark query performance of bulk-loaded tree
    let pts = benchmark_dataset();
    let srtree = SRTree::euclidean(&pts).expect("Failed to build SRTree");
    group.bench_function("bulk-loading", |bencher| {
        bencher.iter(|| {
            for point in &query_points {
                srtree.query(point, K);
            }
        });
    });

    // Linear scan
    let pts = benchmark_dataset();
    group.bench_function("exhaustive", |bencher| {
        bencher.iter(|| {
            for i in 0..query_points.len() {
                // iterate through the points and keep the closest K distances:
                let mut result_heap = BinaryHeap::new();
                for j in 0..pts.len() {
                    result_heap.push(Neighbor::new(
                        OrderedFloat(euclidean_squared(&query_points[i], &pts[j])),
                        j,
                    ));
                    if result_heap.len() > K {
                        result_heap.pop();
                    }
                }
            }
        });
    });
}

pub fn benchmark(criterion: &mut Criterion) {
    build(criterion);
    query(criterion);
}
