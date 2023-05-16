use srtree::{Params, SRTree};

fn main() {
    let params = Params::new(7, 15, 7, true).unwrap();
    let mut tree: SRTree<f64> = SRTree::with_params(params);
    tree.insert(&[0., 0.], 0);
    tree.insert(&[1., 1.], 1);
    tree.insert(&[2., 2.], 2);
    tree.insert(&[3., 3.], 3);
    tree.insert(&[4., 4.], 4);

    let (indices, distances) = tree.query(&[8., 8.], 3);
    println!("{indices:?}"); // [4, 3, 2] (sorted by distance)
}
