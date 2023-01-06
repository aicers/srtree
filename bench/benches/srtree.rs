use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ordered_float::OrderedFloat;
use srtree::{Params, SRTree};
use std::{collections::BinaryHeap, time::Duration};

fn euclidean_squared(point1: &[f64], point2: &[f64]) -> f64 {
    if point1.len() != point2.len() {
        return f64::INFINITY;
    }
    let mut distance = 0.;
    for i in 0..point1.len() {
        distance += (point1[i] - point2[i]).powi(2);
    }
    distance
}

fn bench_exhaustive<const D: usize>(pts: &[[f64; D]], search_points: &[[f64; D]], k: usize) {
    for search_point in search_points {
        // iterate through the points and keep the closest K distances:
        let mut result_heap = BinaryHeap::new();
        for point in pts.iter() {
            result_heap.push(OrderedFloat(euclidean_squared(search_point, point)));
            if result_heap.len() > k {
                result_heap.pop();
            }
        }
    }
}

fn bench_rtree<const D: usize>(pts: &Vec<[f64; D]>, search_points: &Vec<[f64; D]>, k: usize) {
    let mut tree = rtree_rs::RTree::new();
    for i in 0..pts.len() {
        tree.insert(rtree_rs::Rect::new(pts[i], pts[i]), i);
    }

    for i in 0..search_points.len() {
        let mut count = 0;
        let target = rtree_rs::Rect::new(search_points[i], search_points[i]);
        while let Some(_) = tree.nearby(|rect, _| rect.box_dist(&target)).next() {
            count += 1;
            if count == k {
                break;
            }
        }
        assert_eq!(count, k);
    }
}

// Rstar supports max 9 dimensions by default
fn bench_rstar(pts: &Vec<[f64; 9]>, search_points: &Vec<[f64; 9]>, k: usize) {
    let mut tree = rstar::RTree::new();
    for i in 0..pts.len() {
        tree.insert(pts[i]);
    }

    for i in 0..search_points.len() {
        let mut count = 0;
        while let Some(_) = tree.nearest_neighbor_iter(&search_points[i]).next() {
            count += 1;
            if count == k {
                break;
            }
        }
        assert_eq!(count, k);
    }
}

fn bench_srtree<const D: usize>(
    dimension: usize,
    pts: &Vec<[f64; D]>,
    search_points: &Vec<[f64; D]>,
    k: usize,
) {
    let max_elements = 21;
    let min_elements = 10;
    let reinsert_count = min_elements;
    let params = Params::new(min_elements, max_elements, reinsert_count, true).unwrap();
    let mut tree = SRTree::new(dimension, params);
    for point in pts {
        tree.insert(point);
    }

    for point in search_points {
        tree.query(point, k);
    }
}

fn benchmark_with_uniform_dataset(criterion: &mut Criterion) {
    const N: usize = 10_000; // # of training points
    const D: usize = 9; // dimension of each point
    const M: usize = 100; // # of search points
    const K: usize = 100; // # of nearest neighbors to search

    println!();
    println!("Number of training points:   {:?}", N);
    println!("Dimension of each point:     {:?}", D);
    println!("Number of query points:      {:?}", M);
    println!("Number of nearest neighbors: {:?}", K);

    let mut pts = Vec::new();
    for _ in 0..N {
        let mut point = [0.; D];
        for item in point.iter_mut().take(D) {
            *item = rand::random::<f64>() * 1_000_000.;
        }
        pts.push(point);
    }

    let mut search_points = Vec::new();
    for _ in 0..M {
        let mut point = [0.; D];
        for item in point.iter_mut().take(D) {
            *item = rand::random::<f64>() * 1_000_000.;
        }
        search_points.push(point);
    }

    criterion.bench_function("exhaustive search", |bencher| {
        bencher.iter(|| {
            bench_exhaustive(&pts, &search_points, K);
        });
    });

    criterion.bench_function("rtree", |bencher| {
        bencher.iter(|| {
            bench_rtree(&pts, &search_points, K);
        });
    });

    criterion.bench_function("rstar", |bencher| {
        bencher.iter(|| {
            bench_rstar(&pts, &search_points, K);
        });
    });

    criterion.bench_function("srtree", |bencher| {
        bencher.iter(|| {
            bench_srtree(D, &pts, &search_points, K);
        });
    });
}

fn benchmark_with_cluster_dataset(criterion: &mut Criterion) {
    const N: usize = 1000; // # of clusters
    const W: usize = 1000; // # of points in a cluster
    const D: usize = 100; // dimension of each point
    const M: usize = 100; // # of search points
    const K: usize = 100; // # of nearest neighbors to search

    println!();
    println!("Number of clusters:   {:?}", N);
    println!("Number of points per cluster:   {:?}", W);
    println!("Dimension of each point:     {:?}", D);
    println!("Number of query points:      {:?}", M);
    println!("Number of nearest neighbors: {:?}", K);

    let mut pts = Vec::new();
    let mut start = 0.;
    for _ in 0..N {
        for _ in 0..W {
            let mut point = [0.; D];
            for item in point.iter_mut().take(D) {
                *item = start + rand::random::<f64>() * 8000.;
            }
            pts.push(point);
        }
        start += 10_000.;
    }

    let mut search_points = Vec::new();
    for _ in 0..M {
        let mut point = [0.; D];
        for item in point.iter_mut().take(D) {
            *item = rand::random::<f64>() * 1_000_000.;
        }
        search_points.push(point);
    }

    criterion.bench_function("exhaustive search", |bencher| {
        bencher.iter(|| {
            bench_exhaustive(&pts, &search_points, K);
        });
    });

    criterion.bench_function("rtree", |bencher| {
        bencher.iter(|| {
            bench_rtree(&pts, &search_points, K);
        });
    });

    criterion.bench_function("srtree", |bencher| {
        bencher.iter(|| {
            bench_srtree(D, &pts, &search_points, K);
        });
    });
}

criterion_group!{
    name = benches;
    config = Criterion::default().measurement_time(Duration::from_secs(100));
    targets = benchmark_with_uniform_dataset
}
criterion_main!(benches);
