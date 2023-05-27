use rand::prelude::*;
use srtree::SRTree;

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
    const DIMENSION: usize = 2;
    let number_of_points = 1000;
    let radius: f64 = 10.0;

    let mut rng = rand::thread_rng();
    let mut pts = Vec::new();
    for _ in 0..number_of_points {
        let mut point_coords = Vec::new();
        for _ in 0..DIMENSION {
            let x: f64 = rng.gen::<f64>() * 100.;
            point_coords.push(x);
        }
        pts.push(point_coords);
    }

    let bulk_tree = SRTree::new(&pts).expect("Failed to build SRTree");
    let points = pts.clone();
    for p in pts.iter() {
        // Bulk-loaded SRTree nearest neighbors
        let mut result = bulk_tree.query_radius(p, radius);
        result.sort();

        // Brute-force
        let mut brute_force_result = Vec::new();
        for (index, point) in points.iter().enumerate() {
            if euclidean(point, p) <= radius {
                brute_force_result.push(index);
            }
        }

        assert_eq!(result.len(), brute_force_result.len());

        for i in 0..result.len() {
            assert_eq!(result[i], brute_force_result[i]);
        }
    }
}
