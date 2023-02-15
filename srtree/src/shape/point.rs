use ordered_float::Float;
use std::{
    fmt::Debug,
    ops::{AddAssign, DivAssign, MulAssign, SubAssign},
};

#[derive(Debug, Clone)]
pub struct Point<T> {
    coords: Vec<T>,
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

    pub fn dimension(&self) -> usize {
        self.coords.len()
    }

    pub fn coord_at(&self, index: usize) -> T {
        self.coords[index]
    }

    pub fn set_coord_at(&mut self, index: usize, value: T) {
        self.coords[index] = value;
    }

    pub fn coords(&self) -> &Vec<T> {
        &self.coords
    }
}
