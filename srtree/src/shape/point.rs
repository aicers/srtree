use crate::measure::distance::{euclidean, euclidean_squared};
use ordered_float::Float;

#[derive(Clone)]
pub struct Point<T> {
    pub coords: Vec<T>,
    pub radius: T,
    pub index: usize,
}

impl<T> Point<T>
where
    T: Float + Send + Sync,
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

    pub fn distance_squared(&self, other: &Point<T>) -> T {
        euclidean_squared(self, other)
    }

    pub fn distance(&self, other: &Point<T>) -> T {
        euclidean(self, other)
    }
}
