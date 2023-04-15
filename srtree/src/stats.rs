use num_traits::Float;
use std::{
    fmt::Debug,
    ops::{AddAssign, DivAssign, MulAssign, SubAssign},
};

use crate::node::Node;

static mut num_visited_nodes: usize = 0;
static mut num_visited_leaves: usize = 0;
static mut num_compared_nodes: usize = 0;
static mut num_compared_leaves: usize = 0;

pub fn reset_stats() {
    unsafe {
        num_visited_nodes = 0;
        num_visited_leaves = 0;
        num_compared_nodes = 0;
        num_compared_leaves = 0;
    }
}

pub fn inc_compared_leaves() {
    if STATS_ENABLED {
        unsafe {
            num_compared_leaves += 1;
        }
    }
}

pub fn inc_compared_nodes() {
    if STATS_ENABLED {
        unsafe {
            num_compared_nodes += 1;
        }
    }
}

pub fn inc_visited_leaves() {
    if STATS_ENABLED {
        unsafe {
            num_visited_leaves += 1;
        }
    }
}

pub fn inc_visited_nodes() {
    if STATS_ENABLED {
        unsafe {
            num_visited_nodes += 1;
        }
    }
}

pub fn print_stats<T>(root: &Node<T>)
where
    T: Float + AddAssign + SubAssign + MulAssign + DivAssign + Debug + Copy,
{
    if STATS_ENABLED {
        unsafe {
            println!("----------------------");
            println!("Visited leaves:  {}", num_visited_leaves);
            println!("Compared leaves: {}", num_compared_leaves);
            println!("Visited nodes:   {}", num_visited_nodes);
            println!("Compared nodes:  {}", num_compared_nodes + 1); // including root
            println!("Total leaves:    {}", root.leaf_count());
            println!(
                "Total nodes:     {} (including leaf nodes)",
                root.node_count()
            );
            println!("Tree height:     {}", root.get_height());
            println!("----------------------");
            print_node(&root);
        }
    }
}

pub fn print_node<T>(node: &Node<T>)
where
    T: Float + AddAssign + SubAssign + MulAssign + DivAssign + Debug + Copy,
{
    if STATS_ENABLED {
        if !node.is_leaf() {
            println!(
                "Node: {:?}, size = {:?}",
                node.get_height(),
                node.nodes().len()
            );
            for child in node.nodes() {
                print_node(child);
            }
        }
    }
}

// Step 1: Enable stats
const STATS_ENABLED: bool = false;

#[cfg(test)]
mod tests {
    use crate::{Params, SRTree};
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
        const D: usize = 16; // dimension
        const N: usize = 10000; // number of points
        let pts = generate_uniform_dataset(N, D);
        let tree = SRTree::bulk_load(&pts, Params::default_params());
        tree.query(&pts[0], 15);
    }
}
