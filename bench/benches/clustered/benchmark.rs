use super::utils::{
    audio_dataset, clustered_dataset, darpa_audio_dataset, dns_dataset, euclidean_squared,
    glove50D_dataset, world_cities,
};
use crate::neighbor::Neighbor;
use criterion::{black_box, Criterion};
use ndarray::{ArrayBase, ArrayView, CowRepr};
use ordered_float::OrderedFloat;
use petal_neighbors::BallTree;
use rand::{rngs::StdRng, seq::SliceRandom, SeedableRng};
use srtree::{Params, SRTree};
use std::collections::BinaryHeap;

// Note:
// R-tree (https://github.com/tidwall/rtree.rs) does not support bulk loading
const K: usize = 15; // number of nearest neighbors

fn benchmark_dataset() -> Vec<[f64; 2]> {
    world_cities()
}

fn query_dataset(n: usize) -> Vec<[f64; 2]> {
    let mut rng = StdRng::from_seed(*b"PiH6Xi3GBBXhTK6UsXJYngHaF3fx4aYS");
    let mut pts = benchmark_dataset();
    pts.shuffle(&mut rng);
    pts.truncate(n);
    pts
}

pub fn build_and_query(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("clustered");
    group.sample_size(10);

    let query_pts = query_dataset(1000);

    // R*tree (https://github.com/georust/rstar)
    group.bench_function("rstar-seq", |bencher| {
        bencher.iter(|| {
            let pts = benchmark_dataset();
            let mut rstar = rstar::RTree::new();
            for (_, point) in pts.iter().enumerate() {
                rstar.insert(point.clone());
            }
            for i in 0..query_pts.len() {
                let mut count = 0;
                let mut iter = rstar.nearest_neighbor_iter(&query_pts[i]);
                while let Some(_) = iter.next() {
                    count += 1;
                    if count == K {
                        break;
                    }
                }
            }
        });
    });

    // R*tree (bulk-loaded)
    group.bench_function("rstar", |bencher| {
        bencher.iter(|| {
            let pts = benchmark_dataset();
            let rstar = rstar::RTree::bulk_load(pts.clone());
            for i in 0..query_pts.len() {
                let mut count = 0;
                let mut iter = rstar.nearest_neighbor_iter(&query_pts[i]);
                while let Some(_) = iter.next() {
                    count += 1;
                    if count == K {
                        break;
                    }
                }
            }
        });
    });

    // Ball-tree (https://github.com/petabi/petal-neighbors)
    group.bench_function("ball-tree", |bencher| {
        bencher.iter(|| {
            let pts = benchmark_dataset();
            let n = black_box(pts.len());
            let dim = black_box(pts[0].len());
            let data: Vec<f64> = pts.clone().into_iter().flatten().collect();
            let array = ArrayView::from_shape((n, dim), &data).unwrap();
            let tree = BallTree::euclidean(array).expect("`array` is not empty");
            for point in &query_pts {
                tree.query(
                    &<ArrayBase<CowRepr<f64>, _> as From<&[f64]>>::from(point),
                    K,
                );
            }
        });
    });

    // Sequentially built SR-tree
    group.bench_function("srtree-seq", |bencher| {
        bencher.iter(|| {
            let pts = benchmark_dataset();
            let pts: Vec<Vec<f64>> = pts.into_iter().map(|p| p.to_vec()).collect();
            let mut srtree = SRTree::new();
            for (i, point) in pts.iter().enumerate() {
                srtree.insert(point, i);
            }
            for point in &query_pts {
                srtree.query(point, K);
            }
        });
    });

    // SR-tree
    group.bench_function("srtree-bulk", |bencher| {
        bencher.iter(|| {
            let pts = benchmark_dataset();
            let pts: Vec<Vec<f64>> = pts.into_iter().map(|p| p.to_vec()).collect();
            let srtree = SRTree::bulk_load(&pts, Params::default_params());
            for point in &query_pts {
                srtree.query(point, K);
            }
        });
    });

    // Linear scan
    group.bench_function("exhaustive", |bencher| {
        bencher.iter(|| {
            let pts = benchmark_dataset();
            for i in 0..query_pts.len() {
                // iterate through the points and keep the closest K distances:
                let mut result_heap = BinaryHeap::new();
                for j in 0..pts.len() {
                    result_heap.push(Neighbor::new(
                        OrderedFloat(euclidean_squared(&query_pts[i], &pts[j])),
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
