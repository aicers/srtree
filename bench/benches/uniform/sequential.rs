use std::collections::BinaryHeap;
use criterion::{black_box, Criterion};
use ordered_float::OrderedFloat;
use srtree::{Params, SRTree};

use crate::uniform::utils::{uniform_dataset, euclidean_squared};

// Note:
// Ball-tree (https://github.com/petabi/petal-neighbors) does not support dynamic insertions

fn build(criterion: &mut Criterion) {
    const D: usize = 2; // dimension
    let n: usize = black_box(2000); // number of points
    let mut group = criterion.benchmark_group("build");

    // R-tree (https://github.com/tidwall/rtree.rs)
    group.bench_function("rtree", |bencher| {
        bencher.iter(|| {
            let pts: Vec<[f64; D]> = uniform_dataset(n);
            let mut rtree = rtree_rs::RTree::new();
            for i in 0..pts.len() {
                rtree.insert(rtree_rs::Rect::new(pts[i], pts[i]), i);
            }
        });
    });

    // R*tree (https://github.com/georust/rstar)
    group.bench_function("rstar", |bencher| {
        bencher.iter(|| {
            let pts: Vec<[f64; D]> = uniform_dataset(n);
            let mut rstar = rstar::RTree::new();
            for i in 0..pts.len() {
                rstar.insert(pts[i]);
            }
        });
    });

    // SR-tree
    group.bench_function("srtree", |bencher| {
        bencher.iter(|| {
            let pts: Vec<[f64; D]> = uniform_dataset(n);
            let max_elements = 21;
            let min_elements = 9;
            let reinsert_count = 7;
            let params = Params::new(min_elements, max_elements, reinsert_count, true).unwrap();
            let mut srtree = SRTree::new(params);
            for i in 0..pts.len() {
                srtree.insert(&pts[i].to_vec(), i);
            }
        });
    });
}

fn query(criterion: &mut Criterion) {
    const D: usize = 2; // dimension
    let n: usize = black_box(2000); // number of points
    let k: usize = black_box(15); // number of nearest neighbors
    let pts: Vec<[f64; D]> = uniform_dataset(n);
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
    let max_elements = 21;
    let min_elements = 9;
    let reinsert_count = 7;
    let params = Params::new(min_elements, max_elements, reinsert_count, true).unwrap();
    let mut srtree = SRTree::new(params);
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
    group.bench_function("exhaustive", |bencher| {
        bencher.iter(|| {
            for query_point in &pts {
                // iterate through the points and keep the closest K distances:
                let mut result_heap = BinaryHeap::new();
                for point in pts.iter() {
                    result_heap.push(OrderedFloat(euclidean_squared(query_point, point)));
                    if result_heap.len() > k {
                        result_heap.pop();
                    }
                }
            }
        });
    });
}

pub fn sequential_benchmark(criterion: &mut Criterion) {
    build(criterion);
    query(criterion);
}
