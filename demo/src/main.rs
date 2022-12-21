use srtree::{Params, SRTree};

fn main() {
    let params = Params::new(7, 15, 7, true).unwrap();
    let mut tree: SRTree<f64> = SRTree::new(2, params);
    let number_of_points = 100;

    for i in 0..number_of_points {
        let x = f64::from(i);
        let y = f64::from(i);
        tree.insert(&[x, y]);
    }

    let neighbors = tree.query(&[0., 0.], 2);
    println!("{:?}", neighbors[0]); // [0., 0.]
    println!("{:?}", neighbors[1]); // [1., 1.]
}
