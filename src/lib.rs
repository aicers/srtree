use num_traits::Float;
use num_traits::FromPrimitive;
use num_traits::Zero;
use std::ops::AddAssign;
use std::ops::DivAssign;
use std::ops::MulAssign;
use std::ops::SubAssign;

mod distance;
mod rect;
mod sphere;
mod node;
use crate::node::{Data, Node};

pub struct SRTree<T, const dimension: usize> {
    root: Option<node::Node<T, dimension>>,
    m: usize, // min number of entries in a node / leaf
    M: usize  // max number of entries in a node / leaf
}


impl <T, const dimension: usize> SRTree<T, dimension>
where
    T: Float + Zero + FromPrimitive + AddAssign + SubAssign + DivAssign + MulAssign,
{
    pub fn new(m: usize, M: usize) -> SRTree<T, dimension>{
        SRTree { root: None, m, M }
    }
}
