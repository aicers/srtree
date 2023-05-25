use crate::measure::distance::{euclidean, euclidean_squared};
use ordered_float::Float;

#[derive(Clone)]
pub struct Point<T> {
    pub coords: Vec<T>,
    pub radius: T,
    pub index: usize,
    pub parent_index: usize,
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
            parent_index: usize::MAX,
        }
    }

    pub fn distance_squared(&self, other: &Point<T>) -> T {
        euclidean_squared(self, other)
    }

    pub fn distance(&self, other: &Point<T>) -> T {
        euclidean(self, other)
    }

    pub fn dimension(&self) -> usize {
        self.coords.len()
    }
}
