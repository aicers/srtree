use super::data::world_cities_dataset;
use criterion::{black_box, Criterion};
use ndarray::{ArrayBase, ArrayView, CowRepr};
use petal_neighbors::BallTree;
use srtree::{Params, SRTree};

// Note:
// R-tree (https://github.com/tidwall/rtree.rs) does not support bulk loading

fn build(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("build");

    // R*tree (https://github.com/georust/rstar)
    group.bench_function("rstar", |bencher| {
        bencher.iter(|| {
            let pts = world_cities_dataset();
            rstar::RTree::bulk_load(pts)
        });
    });

    // Ball-tree (https://github.com/petabi/petal-neighbors)
    group.bench_function("ball-tree", |bencher| {
        bencher.iter(|| {
            let pts = world_cities_dataset();
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
            let pts = world_cities_dataset();
            let pts: Vec<Vec<f64>> = pts.into_iter().map(|p| p.to_vec()).collect();
            SRTree::bulk_load(&pts, Params::default_params())
        });
    });
}

fn query(criterion: &mut Criterion) {
    let pts = world_cities_dataset();
    const D: usize = 2;
    let n = black_box(pts.len());
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
    let array = ArrayView::from_shape((n, D), &data).unwrap();
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
}

pub fn world_cities_benchmark(criterion: &mut Criterion) {
    build(criterion);
    query(criterion);
}
