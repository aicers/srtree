use srtree::{Metric, SRTree};

struct Manhattan;
impl Metric<f64> for Manhattan {
    fn distance(&self, point1: &[f64], point2: &[f64]) -> f64 {
        point1.iter().zip(point2).map(|(a, b)| (a - b).abs()).sum()
    }

    fn distance_squared(&self, _: &[f64], _: &[f64]) -> f64 {
        0.
    }
}

fn main() {
    let points = vec![
        vec![0., 0.],
        vec![1., 1.],
        vec![2., 2.],
        vec![3., 3.],
        vec![4., 4.],
    ];

    // Build a tree with Euclidean distance
    let tree = SRTree::euclidean(&points).expect("Failed to build SRTree");
    let (indices, distances) = tree.query(&[8., 8.], 3);
    println!("{indices:?}"); // [4, 3, 2] (sorted by distance)
    println!("{distances:?}");

    // Build a tree with Manhattan distance
    let tree = SRTree::default(&points, Manhattan).expect("Failed to build SRTree");
    let (indices, distances) = tree.query(&[8., 8.], 3);
    println!("{indices:?}"); // [4, 3, 2] (sorted by distance)
    println!("{distances:?}"); // [8., 10., 12.]
}
