use std::collections::BinaryHeap;
use criterion::{black_box, Criterion};
use ordered_float::OrderedFloat;
use srtree::SRTree;

use crate::uniform::utils::{uniform_dataset, euclidean_squared};


// Note:
// Ball-tree (https://github.com/petabi/petal-neighbors) does not support dynamic insertions
const N: usize = 10000; // number of points
const D: usize = 9; // dimension

fn build(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("build");

    // R-tree (https://github.com/tidwall/rtree.rs)
    group.bench_function("rtree", |bencher| {
        bencher.iter(|| {
            let pts: Vec<[f64; D]> = uniform_dataset(N);
            let mut rtree = rtree_rs::RTree::new();
            for i in 0..pts.len() {
                rtree.insert(rtree_rs::Rect::new(pts[i], pts[i]), i);
            }
        });
    });

    // R*tree (https://github.com/georust/rstar)
    group.bench_function("rstar", |bencher| {
        bencher.iter(|| {
            let pts: Vec<[f64; D]> = uniform_dataset(N);
            let mut rstar = rstar::RTree::new();
            for i in 0..pts.len() {
                rstar.insert(pts[i]);
            }
        });
    });

    // SR-tree
    group.bench_function("srtree", |bencher| {
        bencher.iter(|| {
            let pts: Vec<[f64; D]> = uniform_dataset(N);
            let mut srtree = SRTree::new();
            for i in 0..pts.len() {
                srtree.insert(&pts[i].to_vec(), i);
            }
        });
    });
}

fn query(criterion: &mut Criterion) {
    let pts: Vec<[f64; D]> = uniform_dataset(N);
    let k: usize = black_box(15); // number of nearest neighbors
    let mut group = criterion.benchmark_group("query");

    // R-tree (https://github.com/tidwall/rtree.rs)
    let mut rtree = rtree_rs::RTree::new();
    for i in 0..pts.len() {
        rtree.insert(rtree_rs::Rect::new(pts[i], pts[i]), i);
    }
    group.bench_function("rtree", |bencher| {
        bencher.iter(|| {
            for i in 0..pts.len() {
                let mut count = 0;
                let target = rtree_rs::Rect::new(pts[i], pts[i]);
                let mut iter = rtree.nearby(|rect, _| rect.box_dist(&target));
                while let Some(_) = iter.next() {
                    count += 1;
                    if count == k {
                        break;
                    }
                }
            }
        });
    });

    // R*tree (https://github.com/georust/rstar)
    let mut rstar = rstar::RTree::new();
    for point in pts.clone() {
        rstar.insert(point);
    }
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

    // SR-tree
    let mut srtree = SRTree::new();
    for i in 0..pts.len() {
        srtree.insert(&pts[i].to_vec(), i);
    }
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

pub fn sequential_benchmark(criterion: &mut Criterion) {
    build(criterion);
    query(criterion);
}
