use ordered_float::OrderedFloat;
use rand::prelude::*;
use srtree::{Params, SRTree};

pub fn euclidean(point1: &[f64], point2: &[f64]) -> f64 {
    if point1.len() != point2.len() {
        return f64::INFINITY;
    }
    let mut distance = 0.;
    for i in 0..point1.len() {
        distance += (point1[i] - point2[i]).powi(2);
    }
    distance.sqrt()
}

#[test]
fn test_with_random_points() {
    let params = Params::new(7, 15, 7, true).unwrap();
    let mut tree: SRTree<f64> = SRTree::new(2, params);
    let number_of_points = 100;
    let mut rng = rand::thread_rng();

    let mut inserted = 0;
    let mut all_points = Vec::new();
    while inserted < number_of_points {
        let x: f64 = 1000. * rng.gen::<f64>();
        let y: f64 = 1000. * rng.gen::<f64>();
        all_points.push(vec![x, y]);
        tree.insert(&[x, y]);
        inserted += 1;
    }

    let mut points = all_points.clone();
    for p in all_points.iter() {
        let k = 10;
        let result = tree.query(&p, 10);
        points.sort_by_key(|a| OrderedFloat(euclidean(p, a)));

        for i in 0..k {
            assert_eq!(result[i], points[i]);
        }
    }
}
