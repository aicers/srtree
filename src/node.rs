use num_traits::Float;
use num_traits::FromPrimitive;
use num_traits::Zero;
use std::ops::AddAssign;
use std::ops::DivAssign;
use std::ops::MulAssign;
use std::ops::SubAssign;
use crate::rect::Rect;
use crate::sphere::Sphere;


pub enum Data<T> {
    Point(Vec<T>),
    Nodes(Vec<Node<T>>)
}

pub struct Node<T>{
    rect: Rect<T>,
    sphere: Sphere<T>,
    data: Data<T>,
    number_of_points: usize
}

impl<T> Node<T>
where
    T: Float + Zero + FromPrimitive + AddAssign + SubAssign + DivAssign + MulAssign,
{
    pub fn from_point(point: Vec<T>) -> Node<T> {
        Node { 
            rect: Rect::from_point(&point).unwrap(), 
            sphere: Sphere::from_point(&point), 
            data: Data::Point(point), 
            number_of_points: 1 
        }
    }
}
