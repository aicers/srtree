use crate::uniform::data::{euclidean_squared, uniform_dataset};
use criterion::{black_box, Criterion};
use ndarray::{ArrayBase, ArrayView, CowRepr};
use petal_neighbors::BallTree;
use srtree::SRTree;

// Note:
const N: usize = 2000; // number of points
const D: usize = 8; // dimension
const R: f64 = 10.; // radius

pub fn build_and_query(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("uniform");
    group.sample_size(10);

    // SR-tree
    group.bench_function("srtree", |bencher| {
        bencher.iter(|| {
            let pts: Vec<[f64; D]> = uniform_dataset(N);
            let pts: Vec<Vec<f64>> = pts.into_iter().map(|p| p.to_vec()).collect();
            let srtree = SRTree::new(&pts);
            for point in &pts {
                srtree.query_radius(point, R);
            }
        });
    });

    // Ball-tree (https://github.com/petabi/petal-neighbors)
    group.bench_function("ball-tree", |bencher| {
        bencher.iter(|| {
            let pts: Vec<[f64; D]> = uniform_dataset(N);
            let n = black_box(pts.len());
            let dim = black_box(D);
            let data: Vec<f64> = pts.clone().into_iter().flatten().collect();
            let array = ArrayView::from_shape((n, dim), &data).unwrap();
            let tree = BallTree::euclidean(array).expect("`array` is not empty");
            for point in &pts {
                tree.query_radius(
                    &<ArrayBase<CowRepr<f64>, _> as From<&[f64]>>::from(point),
                    R,
                );
            }
        });
    });

    // Linear scan
    group.bench_function("exhaustive", |bencher| {
        bencher.iter(|| {
            let pts: Vec<[f64; D]> = uniform_dataset(N);
            for i in 0..pts.len() {
                // iterate through the points and keep the closest K distances:
                let mut result = Vec::new();
                for j in 0..pts.len() {
                    let distance = euclidean_squared(&pts[i], &pts[j]).sqrt();
                    if distance <= R {
                        result.push(j);
                    }
                }
            }
        });
    });
}
