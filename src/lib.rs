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

pub struct SRTree<T> {
    root: Option<Node<T>>,
    min_number_of_elements: usize,
    max_number_of_elements: usize
}


impl <T> SRTree<T>
where
    T: Float + Zero + FromPrimitive + AddAssign + SubAssign + DivAssign + MulAssign,
{
    pub fn new(min_number_of_elements: usize, max_number_of_elements: usize) -> SRTree<T>{
        SRTree { root: None, min_number_of_elements, max_number_of_elements }
    }
}
