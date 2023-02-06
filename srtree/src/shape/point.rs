use std::{
    fmt::Debug,
    ops::{AddAssign, DivAssign, MulAssign, SubAssign},
};

use ordered_float::Float;

use crate::measure::distance::{euclidean, euclidean_squared};

pub struct Point<T, D> {
    pub coords: Vec<T>,
    pub data: D,
}

impl <T, D> Point<T, D>
where
    T: Debug + Copy + Float + AddAssign + SubAssign + MulAssign + DivAssign,
{
    pub fn new(coords: Vec<T>, data: D) -> Point<T, D> {
        Point { coords, data }
    }

    pub fn distance(&self, point: &Point<T, D>) -> T {
        euclidean(&self.coords, &point.coords)
    }

    pub fn distance_squared(&self, point: &Point<T, D>) -> T {
        euclidean_squared(&self.coords, &point.coords)
    }
}