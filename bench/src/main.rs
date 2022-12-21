use srtree::{Params, SRTree};

fn test_srtree() {
    const N: usize = 1_000_000; // # of training points
    const D: usize = 9; // dimension of each point
    const M: usize = 1000; // # of search points
    const K: usize = 1; // # of nearest neighbors to search

    println!();
    println!("Number of training points:   {:?}", N);
    println!("Dimension of each point:     {:?}", D);
    println!("Number of query points:      {:?}", M);
    println!("Number of nearest neighbors: {:?}", K);

    let mut pts = Vec::new();
    for _ in 0..N {
        let mut point = [0.; D];
        for item in point.iter_mut().take(D) {
            *item = rand::random::<f64>() * 1_000_000.;
        }
        pts.push(point);
    }

    let mut search_points = Vec::new();
    for _ in 0..M {
        let mut point = [0.; D];
        for item in point.iter_mut().take(D) {
            *item = rand::random::<f64>() * 1_000_000.;
        }
        search_points.push(point);
    }

    println!();
    println!("---- RTree ----");
    let mut tree = rtree_rs::RTree::new();
    print!("insert:        ");
    lotsa::ops(pts.len(), 1, |i, _| {
        tree.insert(rtree_rs::Rect::new(pts[i], pts[i]), i);
    });
    print!("kNN query:     ");
    lotsa::ops(search_points.len(), 1, |i, _| {
        let target = rtree_rs::Rect::new(search_points[i], search_points[i]);
        tree.nearby(|rect, _| rect.box_dist(&target)).next();
    });

    println!();
    println!("---- RStar ----");
    let mut tree = rstar::RTree::new();
    print!("insert:        ");
    lotsa::ops(N, 1, |i, _| {
        tree.insert(pts[i]);
    });
    print!("kNN query:     ");
    lotsa::ops(search_points.len(), 1, |i, _| {
        tree.nearest_neighbor_iter(&search_points[i]).next();
    });

    println!();
    println!("---- SRTree ----");
    let max_elements = 32;
    let min_elements = max_elements * 20 / 100;
    let reinsert_count = min_elements;
    let params = Params::new(min_elements, max_elements, reinsert_count, true).unwrap();
    let mut tree = SRTree::new(D, params);
    print!("insert:        ");
    lotsa::ops(pts.len(), 1, |i, _| {
        tree.insert(&pts[i]);
    });
    print!("kNN query:     ");
    lotsa::ops(search_points.len(), 1, |i, _| {
        tree.query(&search_points[i], K);
    });
}

fn main() {
    test_srtree();
}
