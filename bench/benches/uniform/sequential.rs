use criterion::{black_box, Criterion};
use ordered_float::OrderedFloat;
use priority_queue::PriorityQueue;
use srtree::SRTree;
use std::collections::BinaryHeap;

use crate::uniform::utils::{euclidean_squared, uniform_dataset};

// Note:
// Ball-tree (https://github.com/petabi/petal-neighbors) does not support dynamic insertions
const N: usize = 5000; // number of points
const D: usize = 2; // dimension

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
    let pts: Vec<Vec<f64>> = pts.into_iter().map(|p| p.to_vec()).collect();
    let mut srtree = SRTree::new();
    for i in 0..pts.len() {
        srtree.insert(&pts[i], i);
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
            for i in 0..pts.len() {
                let mut queue = PriorityQueue::new();
                for j in 0..pts.len() {
                    let dist = euclidean_squared(&pts[i], &pts[j]);
                    queue.push(j, OrderedFloat(dist));
                    if queue.len() > k {
                        queue.pop();
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
