use srtree::{SRTree, Params};

fn main() {
    const N: usize = 1_000_000;
    
    let mut pts = Vec::new();
    for _ in 0..N {
        let pt = [
            rand::random::<f64>() * 360.0 - 180.0,
            rand::random::<f64>() * 180.0 - 90.0,
        ];
        pts.push(pt);
    }

    // 1%
    let mut r1 = Vec::new();
    for _ in 0..10_000 {
        let p = 0.01;
        let min = [
            rand::random::<f64>() * 360.0 - 180.0,
            rand::random::<f64>() * 180.0 - 90.0,
        ];
        let max = [min[0] + 360.0 * p, min[1] + 180.0 * p];
        r1.push((min, max));
    }
    // 5%
    let mut r5 = Vec::new();
    for _ in 0..10_000 {
        let p = 0.05;
        let min = [
            rand::random::<f64>() * 360.0 - 180.0,
            rand::random::<f64>() * 180.0 - 90.0,
        ];
        let max = [min[0] + 360.0 * p, min[1] + 180.0 * p];
        r5.push((min, max));
    }
    // 10%
    let mut r10 = Vec::new();
    for _ in 0..10_000 {
        let p = 0.10;
        let min = [
            rand::random::<f64>() * 360.0 - 180.0,
            rand::random::<f64>() * 180.0 - 90.0,
        ];
        let max = [min[0] + 360.0 * p, min[1] + 180.0 * p];
        r10.push((min, max));
    }

    println!(">>> SRTree::new() <<<");
    let max_elements = 32;
    let min_elements = max_elements * 20 / 100;
    let reinsert_count = min_elements;
    let params = Params::new(min_elements, max_elements, reinsert_count, true).unwrap();
    let mut tr = SRTree::new(2, params);
    print!("insert:        ");
    lotsa::ops(pts.len(), 1, |i, _| {
        tr.insert(&pts[i]);
    });
    print!("search-item:   ");
    lotsa::ops(pts.len(), 1, |i, _| {
        for _ in tr.query(&pts[i], 1) {
            break;
        }
    });
    print!("search-1%:     ");
    lotsa::ops(r1.len(), 1, |i, _| {
        for _ in tr.query(&pts[i], 1) {}
    });
    print!("search-5%:     ");
    lotsa::ops(r5.len(), 1, |i, _| {
        for _ in tr.query(&pts[i], 1) {}
    });
    print!("search-10%:    ");
    lotsa::ops(r10.len(), 1, |i, _| {
        for _ in tr.query(&pts[i], 1) {}
    });
    // print!("remove-half:   ");
    // lotsa::ops(pts.len()/2, 1, |i, _| {
    //     tr.remove(rtree_rs::Rect::new(pts[i*2],pts[i*2]), &(i*2)).unwrap();
    // });
    print!("reinsert-half: ");
    lotsa::ops(pts.len()/2, 1, |i, _| {
        tr.insert(&pts[i]);
    });
    print!("search-item:   ");
    lotsa::ops(pts.len(), 1, |i, _| {
        for _ in tr.query(&pts[i], 1) {
            break;
        }
    });
    print!("search-1%:     ");
    lotsa::ops(r1.len(), 1, |i, _| {
        for _ in tr.query(&pts[i], 1) {}
    });
}
