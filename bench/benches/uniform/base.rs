use criterion::Criterion;
use srtree::{Params, SRTree};

use super::utils::uniform_dataset;

// Note:
const N: usize = 2000; // number of points
const D: usize = 8; // dimension
const K: usize = 15; // number of nearest neighbors

fn build(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("build");
    group.sample_size(10);

    // benchmark build performance of sequential building
    group.bench_function("sequential", |bencher| {
        bencher.iter(|| {
            let pts: Vec<[f64; D]> = uniform_dataset(N);
            let pts: Vec<Vec<f64>> = pts.into_iter().map(|p| p.to_vec()).collect();
            let mut srtree = SRTree::new();
            for (i, point) in pts.iter().enumerate() {
                srtree.insert(point, i);
            }
        });
    });

    // benchmark build performance of bulk-loading
    group.bench_function("bulk-loading", |bencher| {
        bencher.iter(|| {
            let pts: Vec<[f64; D]> = uniform_dataset(N);
            let pts: Vec<Vec<f64>> = pts.into_iter().map(|p| p.to_vec()).collect();
            SRTree::bulk_load(&pts, Params::default_params())
        });
    });
}

fn query(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("query");
    group.sample_size(10);

    // benchmark query performance of sequantially-built tree
    let pts: Vec<[f64; D]> = uniform_dataset(N);
    let pts: Vec<Vec<f64>> = pts.into_iter().map(|p| p.to_vec()).collect();
    let mut srtree = SRTree::new();
    for (i, point) in pts.iter().enumerate() {
        srtree.insert(point, i);
    }
    group.bench_function("sequential", |bencher| {
        bencher.iter(|| {
            for point in &pts {
                srtree.query(point, K);
            }
        });
    });

    // benchmark query performance of bulk-loaded tree
    let pts: Vec<[f64; D]> = uniform_dataset(N);
    let pts: Vec<Vec<f64>> = pts.into_iter().map(|p| p.to_vec()).collect();
    let srtree = SRTree::bulk_load(&pts, Params::default_params());
    group.bench_function("bulk-loading", |bencher| {
        bencher.iter(|| {
            for point in &pts {
                srtree.query(point, K);
            }
        });
    });
}

pub fn benchmark(criterion: &mut Criterion) {
    build(criterion);
    query(criterion);
}
