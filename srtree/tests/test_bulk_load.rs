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
    const DIMENSION: usize = 2;
    let number_of_points = 1000;
    let k = 10;

    let mut rng = rand::thread_rng();
    let mut pts = Vec::new();
    for _ in 0..number_of_points {
        let mut point_coords = Vec::new();
        for _ in 0..DIMENSION {
            let x: f64 = rng.gen::<f64>();
            point_coords.push(x);
        }
        pts.push(point_coords);
    }

    let mut sequential_tree = SRTree::new();
    for (index, point) in pts.iter().enumerate() {
        sequential_tree.insert(point, index);
    }

    let bulk_tree = SRTree::bulk_load(&pts, Params::default_params());
    let mut points = pts.clone();
    for p in pts.iter() {
        // Bulk-loaded SRTree nearest neighbors
        let result_bulk = bulk_tree.query(p, k);
        assert_eq!(result_bulk.len(), k);

        // Sequential SRTree nearest neighbors
        let result_sequential = sequential_tree.query(p, k);
        assert_eq!(result_sequential.len(), k);

        // Brute-force
        points.sort_by_key(|a| OrderedFloat(euclidean_squared(a, p)));

        for i in 0..k {
            assert_eq!(pts[result_bulk[i]], points[i]);
            assert_eq!(pts[result_sequential[i]], points[i]);
        }
    }
}
