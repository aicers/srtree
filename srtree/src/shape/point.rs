use ordered_float::Float;
use std::{
    fmt::Debug,
    ops::{AddAssign, DivAssign, MulAssign, SubAssign},
};

use crate::measure::distance::{euclidean, euclidean_squared};

#[derive(Debug, Clone)]
pub struct Point<T> {
    coords: Vec<T>,
    radius: T,
    pub index: usize,
}

impl<T> Point<T>
where
    T: Debug + Copy + Float + AddAssign + SubAssign + MulAssign + DivAssign,
{
    pub fn new(coords: Vec<T>, index: usize) -> Point<T> {
        Point {
            coords,
            radius: T::zero(),
            index,
        }
    }

    #[allow(dead_code)]
    pub fn with_coords(coords: Vec<T>) -> Point<T> {
        Point {
            coords,
            radius: T::zero(),
            index: 0,
        }
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

    pub fn set_radius(&mut self, radius: T) {
        self.radius = radius;
    }

    pub fn radius(&self) -> T {
        self.radius
    }

    pub fn distance_squared(&self, other: &Point<T>) -> T {
        euclidean_squared(self, other)
    }

    pub fn distance(&self, other: &Point<T>) -> T {
        euclidean(self, other)
    }
}
