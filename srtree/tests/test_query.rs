use ordered_float::OrderedFloat;
use rand::prelude::*;
use srtree::{Params, SRTree};

pub fn euclidean_squared(point1: &[f64], point2: &[f64]) -> f64 {
    if point1.len() != point2.len() {
        return f64::INFINITY;
    }
    let mut distance = 0.;
    for i in 0..point1.len() {
        distance += (point1[i] - point2[i]).powi(2);
    }
    distance
}

#[test]
fn test_with_random_points() {
    let number_of_dimensions = 2;
    let number_of_points = 100;
    let k = 10;

    let params = Params::new(3, 7, 3, true).unwrap();
    let mut tree: SRTree<f64> = SRTree::new(params);

    let mut rng = rand::thread_rng();

    let mut all_points = Vec::new();
    for i in 0..number_of_points {
        let mut point_coords = Vec::new();
        for _ in 0..number_of_dimensions {
            let x: f64 = 1000000. * rng.gen::<f64>();
            point_coords.push(x);
        }
        tree.insert(&point_coords, i);
        all_points.push((point_coords, i));
    }

    let mut points = all_points.clone();
    for p in all_points.iter() {
        // SRTree nearest neighbors
        let result = tree.query(&p.0, k);

        // Brute-force
        points.sort_by_key(|a| OrderedFloat(euclidean_squared(&a.0, &p.0)));

        for i in 0..k {
            assert_eq!(result[i], points[i].1);
        }
    }
}
