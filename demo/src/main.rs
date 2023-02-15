use srtree::{Params, SRTree};

fn main() {
    let params = Params::new(7, 15, 7, true).unwrap();
    let mut tree: SRTree<f64> = SRTree::new(params);
    tree.insert(&vec![0., 0.], 0);
    tree.insert(&vec![1., 1.], 1);
    tree.insert(&vec![2., 2.], 2);
    tree.insert(&vec![3., 3.], 3);
    tree.insert(&vec![4., 4.], 4);

    let neighbors = tree.query(&[8., 8.], 3);
    println!("{neighbors:?}"); // [4, 3, 2] (sorted by distance)
}
