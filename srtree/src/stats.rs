use num_traits::Float;
use std::{
    fmt::Debug,
    ops::{AddAssign, DivAssign, MulAssign, SubAssign},
};

use crate::SRTree;

static mut NUM_VISITED_POINTS: usize = 0;
static mut NUM_COMPARED_POINTS: usize = 0;
static mut NUM_VISITED_NODES: usize = 0;
static mut NUM_VISITED_LEAVES: usize = 0;
static mut NUM_COMPARED_NODES: usize = 0;
static mut NUM_COMPARED_LEAVES: usize = 0;

pub fn reset() {
    unsafe {
        NUM_VISITED_POINTS = 0;
        NUM_COMPARED_POINTS = 0;
        NUM_VISITED_NODES = 0;
        NUM_VISITED_LEAVES = 0;
        NUM_COMPARED_NODES = 0;
        NUM_COMPARED_LEAVES = 0;
    }
}

pub fn inc_compared_points(count: usize) {
    if STATS_ENABLED {
        unsafe {
            NUM_COMPARED_POINTS += count;
        }
    }
}

pub fn inc_compared_leaves() {
    if STATS_ENABLED {
        unsafe {
            NUM_COMPARED_LEAVES += 1;
        }
    }
}

pub fn inc_compared_nodes() {
    if STATS_ENABLED {
        unsafe {
            NUM_COMPARED_NODES += 1;
        }
    }
}

pub fn inc_visited_points() {
    if STATS_ENABLED {
        unsafe {
            NUM_VISITED_POINTS += 1;
        }
    }
}

pub fn inc_visited_leaves() {
    if STATS_ENABLED {
        unsafe {
            NUM_VISITED_LEAVES += 1;
        }
    }
}

pub fn inc_visited_nodes() {
    if STATS_ENABLED {
        unsafe {
            NUM_VISITED_NODES += 1;
        }
    }
}

pub fn print<T>(tree: &SRTree<T>)
where
    T: Float + AddAssign + SubAssign + MulAssign + DivAssign + Debug + Copy,
{
    if STATS_ENABLED {
        unsafe {
            println!("----------------------");
            println!("Visited points:  {NUM_VISITED_POINTS}");
            println!("Compared points: {NUM_COMPARED_POINTS}");
            println!("Visited leaves:  {NUM_VISITED_LEAVES}");
            println!("Compared leaves: {NUM_COMPARED_LEAVES}");
            println!("Visited nodes:   {NUM_VISITED_NODES}");
            println!("Compared nodes:  {}", NUM_COMPARED_NODES + 1); // including root
            println!("Total leaves:    {}", tree.leaf_count());
            println!(
                "Total nodes:     {} (including leaf nodes)",
                tree.node_count()
            );
            println!("Tree height:     {}", tree.height());
            println!("----------------------");
        }
    }
}

// Step 1: Enable stats
const STATS_ENABLED: bool = false;

#[cfg(test)]
mod tests {
    use crate::{
        stats::{print, reset},
        Params, SRTree,
    };
    use rand::{rngs::StdRng, Rng, SeedableRng};

    fn generate_uniform_dataset(n: usize, dim: usize) -> Vec<Vec<f64>> {
        let mut rng = StdRng::from_seed(*b"PiH6Xi3GBBXhTK6UsXJYngHaF3fx4aYS");
        let mut pts = Vec::new();
        for _ in 0..n {
            let mut point = Vec::new();
            for _ in 0..dim {
                point.push(rng.gen::<f64>());
            }
            pts.push(point);
        }
        pts
    }

    // Step 2: Run this test separately
    #[test]
    pub fn test_bulk_load() {
        const D: usize = 8; // dimension
        const N: usize = 2000; // number of points
        let pts = generate_uniform_dataset(N, D);
        let tree = SRTree::bulk_load(&pts, Params::default_params());

        reset();
        tree.query(&pts[0], 15);
        print(&tree);
    }
}
