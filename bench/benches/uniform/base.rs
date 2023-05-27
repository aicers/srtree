use criterion::Criterion;
use srtree::SRTree;

use super::data::uniform_dataset;

// Note:
const N: usize = 2000; // number of points
const D: usize = 8; // dimension
const K: usize = 15; // number of nearest neighbors

fn build(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("build");
    group.sample_size(10);

    // benchmark build performance of bulk-loading
    group.bench_function("bulk-loading", |bencher| {
        bencher.iter(|| {
            let pts: Vec<[f64; D]> = uniform_dataset(N);
            let pts: Vec<Vec<f64>> = pts.into_iter().map(|p| p.to_vec()).collect();
            SRTree::new(&pts)
        });
    });
}

fn query(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("query");
    group.sample_size(10);

    // benchmark query performance of bulk-loaded tree
    let pts: Vec<[f64; D]> = uniform_dataset(N);
    let pts: Vec<Vec<f64>> = pts.into_iter().map(|p| p.to_vec()).collect();
    let srtree = SRTree::new(&pts).expect("Failed to build SRTree");
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
