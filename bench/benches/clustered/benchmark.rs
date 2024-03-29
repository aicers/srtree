use crate::utils::{euclidean_squared, LargeNodeRTree, Neighbor};
use criterion::{black_box, Criterion};
use ndarray::{ArrayBase, ArrayView, CowRepr};
use ordered_float::OrderedFloat;
use petal_neighbors::BallTree;
use rand::{rngs::StdRng, seq::SliceRandom, SeedableRng};
use srtree::SRTree;
use std::collections::BinaryHeap;

#[allow(unused)]
use super::data::{
    audio_dataset, darpa_audio_dataset, dns_dataset, glove100d_dataset, glove50d_dataset,
    home_dataset, world_cities_dataset,
};

// Note:
// R-tree (https://github.com/tidwall/rtree.rs) does not support bulk loading
const K: usize = 15; // number of nearest neighbors

fn benchmark_dataset() -> Vec<[f64; 2]> {
    world_cities_dataset()
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

    // SR-tree
    group.bench_function("srtree", |bencher| {
        bencher.iter(|| {
            let pts = benchmark_dataset();
            let pts: Vec<Vec<f64>> = pts.into_iter().map(|p| p.to_vec()).collect();
            let srtree = SRTree::euclidean(&pts).expect("Failed to build SRTree");
            for point in &query_pts {
                srtree.query(point, K);
            }
        });
    });

    // R*tree (https://github.com/georust/rstar) with bulk loading
    group.bench_function("rstar", |bencher| {
        bencher.iter(|| {
            let pts = benchmark_dataset();
            let rstar: LargeNodeRTree<_> = rstar::RTree::bulk_load_with_params(pts.clone());
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

    // R*tree (https://github.com/georust/rstar)
    group.bench_function("rstar-seq", |bencher| {
        bencher.iter(|| {
            let pts = benchmark_dataset();
            let mut rstar: LargeNodeRTree<_> = rstar::RTree::new_with_params();
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
