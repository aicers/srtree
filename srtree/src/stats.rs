use std::{
    fmt::Debug,
    ops::{AddAssign, DivAssign, MulAssign, SubAssign},
};
use num_traits::Float;

use crate::node::Node;

const STATS_ENABLED: bool = true;
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
where T: Float + AddAssign + SubAssign + MulAssign + DivAssign + Debug + Copy
{
    if STATS_ENABLED {
        unsafe {
            println!("----------------------");
            println!("Visited leaves:  {}", num_visited_leaves);
            println!("Compared leaves: {}", num_compared_leaves);
            println!("Visited nodes:   {}", num_visited_nodes);
            println!("Compared nodes:  {}", num_compared_nodes);
            println!("Total leaves:    {}", root.leaf_count());
            println!("Total nodes:     {}", root.node_count());
        }
    }
}