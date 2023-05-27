use srtree::SRTree;

fn main() {
    let points = vec![
        vec![0., 0.],
        vec![1., 1.],
        vec![2., 2.],
        vec![3., 3.],
        vec![4., 4.],
    ];
    let tree = SRTree::euclidean(&points).expect("Failed to build SRTree");

    let (indices, distances) = tree.query(&[8., 8.], 3);
    println!("{indices:?}"); // [4, 3, 2] (sorted by distance)
    println!("{distances:?}");
}
