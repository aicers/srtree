use srtree::sr::SRTree;

fn main() {
    let mut srtree = SRTree::new(7, 15, 7, true);
    srtree.insert(&[0., 0.]);
    println!("Hello, SRTree!");
}
