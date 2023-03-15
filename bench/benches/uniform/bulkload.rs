use std::collections::BinaryHeap;
use criterion::{black_box, Criterion};
use ndarray::{ArrayBase, ArrayView, CowRepr};
use ordered_float::OrderedFloat;
use petal_neighbors::BallTree;
use srtree::{Params, SRTree};

use crate::uniform::utils::{uniform_dataset, euclidean_squared};

// Note:
// R-tree (https://github.com/tidwall/rtree.rs) does not support bulk loading
const N: usize = 2000; // number of points
const D: usize = 9; // dimension

fn build(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("build");

    // R*tree (https://github.com/georust/rstar)
    group.bench_function("rstar", |bencher| {
        bencher.iter(|| {
            let pts: Vec<[f64; D]> = uniform_dataset(N);
            rstar::RTree::bulk_load(pts)
        });
    });

    // Ball-tree (https://github.com/petabi/petal-neighbors)
    group.bench_function("ball-tree", |bencher| {
        bencher.iter(|| {
            let pts: Vec<[f64; D]> = uniform_dataset(N);
            let n = black_box(pts.len());
            let dim = black_box(2);
            let data: Vec<f64> = pts.clone().into_iter().flatten().collect();
            let array = ArrayView::from_shape((n, dim), &data).unwrap();
            BallTree::euclidean(array).expect("`array` is not empty");
        });
    });

    // SR-tree
    group.bench_function("srtree", |bencher| {
        bencher.iter(|| {
            let pts: Vec<[f64; D]> = uniform_dataset(N);
            let pts: Vec<Vec<f64>> = pts.into_iter().map(|p| p.to_vec()).collect();
            SRTree::bulk_load(&pts, Params::default_params())
        });
    });
}

fn query(criterion: &mut Criterion) {
    let pts: Vec<[f64; D]> = uniform_dataset(N);
    let k: usize = black_box(15); // number of nearest neighbors
    let mut group = criterion.benchmark_group("query");

    // R*tree (https://github.com/georust/rstar)
    let rstar = rstar::RTree::bulk_load(pts.clone());
    group.bench_function("rstar", |bencher| {
        bencher.iter(|| {
            for i in 0..pts.len() {
                let mut count = 0;
                let mut iter = rstar.nearest_neighbor_iter(&pts[i]);
                while let Some(_) = iter.next() {
                    count += 1;
                    if count == k {
                        break;
                    }
                }
            }
        });
    });

    // Ball-tree (https://github.com/petabi/petal-neighbors)
    let data: Vec<f64> = pts.clone().into_iter().flatten().collect();
    let array = ArrayView::from_shape((N, D), &data).unwrap();
    let tree = BallTree::euclidean(array).expect("`array` is not empty");
    group.bench_function("ball-tree", |bencher| {
        bencher.iter(|| {
            for point in &pts {
                tree.query(
                    &<ArrayBase<CowRepr<f64>, _> as From<&[f64]>>::from(point),
                    k,
                );
            }
        });
    });

    // SR-tree
    let pts: Vec<Vec<f64>> = pts.into_iter().map(|p| p.to_vec()).collect();
    let srtree = SRTree::bulk_load(&pts, Params::default_params());
    group.bench_function("srtree", |bencher| {
        bencher.iter(|| {
            for point in &pts {
                srtree.query(point, k);
            }
        });
    });

    // Linear scan
    let mut pts_clone = pts.clone();
    group.bench_function("exhaustive", |bencher| {
        bencher.iter(|| {
            for query_point in &pts {
                pts_clone.select_nth_unstable_by_key(k, |point| OrderedFloat(euclidean_squared(query_point, point)));
            }
        });
    });
}

pub fn bulkload_benchmark(criterion: &mut Criterion) {
    build(criterion);
    query(criterion);
}
