use criterion::{criterion_group, criterion_main, Criterion};
use rand::{rngs::StdRng, SeedableRng, Rng};
use srtree::{Params, SRTree};

const INPUT_SEED: [u8; 32] = *b"PiH6Xi3GBBXhTK6UsXJYngHaF3fx4aYS";
const QUERY_SEED: [u8; 32] = *b"H4NNoe0r5BDtWChfJEgXpXCNaS5IfVxC";

fn generate_points<const D: usize>(n: usize, seed: [u8; 32]) -> Vec<[f64; D]> {
    let mut rng = StdRng::from_seed(seed);
    let mut pts = Vec::new();
    for _ in 0..n {
        let mut point = [0.; D];
        for item in point.iter_mut().take(D) {
            *item = rng.gen::<f64>() * 1_000_000.;
        }
        pts.push(point);
    }
    pts
}

fn insert(criterion: &mut Criterion) {
    const N: usize = 10_000; // # of training points
    const D: usize = 10; // dimension of each point
    let pts: Vec<[f64; D]> = generate_points(N, INPUT_SEED);

    criterion.bench_function("insert", |bencher| {
        bencher.iter(|| {
            let max_elements = 21;
            let min_elements = 7;
            let reinsert_count = min_elements;
            let params = Params::new(min_elements, max_elements, reinsert_count, true).unwrap();
            let mut tree = SRTree::new(D, params);

            for point in &pts {
                tree.insert(point);
            }
        });
    });
}

fn query(criterion: &mut Criterion) {
    const N: usize = 10_000; // # of training points
    const D: usize = 10; // dimension of each point
    let pts: Vec<[f64; D]> = generate_points(N, INPUT_SEED);

    let max_elements = 21;
    let min_elements = 7;
    let reinsert_count = min_elements;
    let params = Params::new(min_elements, max_elements, reinsert_count, true).unwrap();
    let mut tree = SRTree::new(D, params);
    for point in &pts {
        tree.insert(point);
    }

    const M: usize = 100; // # of search points
    const K: usize = 100; // # of nearest neighbors to search
    let query_pts: Vec<[f64; D]> = generate_points(M, QUERY_SEED);

    criterion.bench_function("query", |bencher| {
        bencher.iter(|| {
            for point in &query_pts {
                tree.query(point, K);
            }
        });
    });
}

criterion_group!(benches, insert, query);
criterion_main!(benches);
