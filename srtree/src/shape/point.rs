use ordered_float::Float;
use std::{
    fmt::Debug,
    ops::{AddAssign, DivAssign, MulAssign, SubAssign},
};

pub struct Point<T> {
    pub coords: Vec<T>,
    pub index: usize,
}

impl<T> Point<T>
where
    T: Debug + Copy + Float + AddAssign + SubAssign + MulAssign + DivAssign,
{
    pub fn new(coords: Vec<T>, index: usize) -> Point<T> {
        Point { coords, index }
    }

    #[allow(dead_code)]
    pub fn with_coords(coords: Vec<T>) -> Point<T> {
        Point { coords, index: 0 }
    }
}
