use criterion::{black_box, Criterion};
use ndarray::{ArrayBase, ArrayView, CowRepr};
use ordered_float::OrderedFloat;
use petal_neighbors::BallTree;
use priority_queue::PriorityQueue;
use srtree::{Params, SRTree};

use super::utils::{clustered_dataset, euclidean_squared};

// Note:
// R-tree (https://github.com/tidwall/rtree.rs) does not support bulk loading
const k: usize = 15; // number of nearest neighbors

pub fn build_and_query(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("clustered");

    // R*tree (https://github.com/georust/rstar)
    group.bench_function("rstar", |bencher| {
        bencher.iter(|| {
            let pts: Vec<[f64; 24]> = clustered_dataset();
            let rstar = rstar::RTree::bulk_load(pts.clone());
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
    group.bench_function("ball-tree", |bencher| {
        bencher.iter(|| {
            let pts: Vec<[f64; 24]> = clustered_dataset();
            let n = black_box(pts.len());
            let dim = black_box(pts[0].len());
            let data: Vec<f64> = pts.clone().into_iter().flatten().collect();
            let array = ArrayView::from_shape((n, dim), &data).unwrap();
            let tree = BallTree::euclidean(array).expect("`array` is not empty");
            for point in &pts {
                tree.query(
                    &<ArrayBase<CowRepr<f64>, _> as From<&[f64]>>::from(point),
                    k,
                );
            }
        });
    });

    // SR-tree
    group.bench_function("srtree", |bencher| {
        bencher.iter(|| {
            let pts: Vec<[f64; 24]> = clustered_dataset();
            let pts: Vec<Vec<f64>> = pts.into_iter().map(|p| p.to_vec()).collect();
            let srtree = SRTree::bulk_load(&pts, Params::default_params());
            for point in &pts {
                srtree.query(point, k);
            }
        });
    });

    // Linear scan
    group.bench_function("exhaustive", |bencher| {
        bencher.iter(|| {
            let pts: Vec<[f64; 24]> = clustered_dataset();
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
