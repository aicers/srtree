use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ndarray::ArrayView;
use petal_neighbors::BallTree;
use rand::{rngs::StdRng, seq::SliceRandom, Rng, SeedableRng};
use rand_chacha::ChaChaRng;
use srtree::{Params, SRTree};
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const INPUT_SEED: [u8; 32] = *b"PiH6Xi3GBBXhTK6UsXJYngHaF3fx4aYS";
const QUERY_SEED: [u8; 32] = *b"H4NNoe0r5BDtWChfJEgXpXCNaS5IfVxC";

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

fn uniform_dataset<const D: usize>(n: usize, m: usize) -> (Vec<[f64; D]>, Vec<[f64; D]>) {
    let mut rng = StdRng::from_seed(INPUT_SEED);
    let mut pts = Vec::new();
    for _ in 0..n {
        let mut point = [0.; D];
        for item in point.iter_mut().take(D) {
            *item = rng.gen::<f64>() * 1_000_000.;
        }
        pts.push(point);
    }

    pts.shuffle(&mut ChaChaRng::from_seed(QUERY_SEED));
    let mut query_pts: Vec<[f64; D]> = Vec::new();
    for i in 0..m {
        query_pts.push(pts[i].clone());
    }
    (pts, query_pts)
}

/*
This is the free version of World Cities Dataset that is licensed under Creative Commons Attribution 4.0.
It contains about 43 thousand city records (population, country, location etc.).
See more about the license: https://creativecommons.org/licenses/by/4.0/
Link to the dataset: https://simplemaps.com/data/world-cities.

This function doesn't modify the dataset but only uses locations (latitude & longitude) for benchmarking purposes.
*/
fn world_cities_dataset() -> Vec<[f64; 2]> {
    let mut pts = Vec::new();
    let file = File::open("worldcities.csv");
    if file.is_err() {
        return pts;
    }

    let reader = BufReader::new(file.unwrap());
    let mut skip_csv_header = true;
    for line in reader.lines() {
        if skip_csv_header {
            skip_csv_header = false;
            continue;
        }
        if line.is_ok() {
            let mut location = [f64::INFINITY, f64::INFINITY];
            let line = line.as_ref().unwrap();
            for (i, val) in line.split(",").enumerate() {
                let mut chars = val.chars();
                chars.next();
                chars.next_back();
                let val = chars.as_str();
                if i == 2 || i == 3 {
                    let c: f64 = val.parse().unwrap_or(f64::INFINITY);
                    location[i - 2] = c;
                }
            }
            if location[0] != f64::INFINITY && location[1] != f64::INFINITY {
                pts.push(location);
            }
        }
    }

    pts
}

fn build(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("build");

    // R-tree (https://github.com/tidwall/rtree.rs)
    group.bench_function("rtree", |bencher| {
        bencher.iter(|| {
            let pts = world_cities_dataset();
            let mut rtree = rtree_rs::RTree::new();
            for i in 0..pts.len() {
                rtree.insert(rtree_rs::Rect::new(pts[i], pts[i]), i);
            }
        });
    });

    // R*tree (https://github.com/georust/rstar)
    group.bench_function("rstar", |bencher| {
        bencher.iter(|| {
            let pts = world_cities_dataset();
            let mut rstar = rstar::RTree::new();
            for i in 0..pts.len() {
                rstar.insert(pts[i]);
            }
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
    let pts = world_cities_dataset();
    let n = black_box(pts.len()); // number of points
    let k: usize = black_box(5); // number of nearest neighbors
    let dim: usize = black_box(2); // dimension of each point

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

    // Ball-tree (https://github.com/petabi/petal-neighbors)
    // THIS TAKES ~92 minutes to complete!!! (on MPB 2021 14")
    /*
    let data: Vec<f64> = pts.clone().into_iter().flatten().collect();
    let array = ArrayView::from_shape((n, dim), &data).unwrap();
    let tree = BallTree::euclidean(array).expect("`array` is not empty");
    group.bench_function("ball-tree", |bencher| {
        bencher.iter(|| {
            for point in &pts {
                tree.query(
                    &<ArrayBase<CowRepr<f64>, _> as From<&[f64]>>::from(point),
                    k,
                );
            }
        })
    });
    */

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

    // Exhaustive search
    // THIS TAKES ~36.7 minutes to complete!!! (on MPB 2021 14")
    /*
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
    */
}

criterion_group!(benches, build, query);
criterion_main!(benches);
