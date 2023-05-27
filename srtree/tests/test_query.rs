use ordered_float::OrderedFloat;
use rand::prelude::*;
use srtree::SRTree;

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

    let bulk_tree = SRTree::new(&pts).expect("Failed to build SRTree");
    let mut points = pts.clone();
    for p in pts.iter() {
        // Bulk-loaded SRTree nearest neighbors
        let (bulk_indices, bulk_distances) = bulk_tree.query(p, k);
        assert_eq!(bulk_indices.len(), k);

        // Brute-force
        points.sort_by_key(|a| OrderedFloat(euclidean_squared(a, p)));

        for i in 0..k {
            let distance_brute_force = euclidean_squared(&points[i], p).sqrt();
            assert_eq!(bulk_distances[i], distance_brute_force);
        }
    }
}
