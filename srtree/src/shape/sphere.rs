use crate::measure::distance::euclidean;
use ordered_float::Float;
use std::{
    fmt::Debug,
    ops::{AddAssign, DivAssign, MulAssign, SubAssign},
};

#[derive(Debug)]
pub struct Sphere<T> {
    pub center: Vec<T>,
    pub radius: T,
}

impl<T> Sphere<T>
where
    T: Debug + Copy + Float + AddAssign + SubAssign + MulAssign + DivAssign,
{
    pub fn new(center: Vec<T>, radius: T) -> Sphere<T> {
        Sphere { center, radius }
    }

    pub fn from_point(point: &[T]) -> Sphere<T> {
        Sphere::new(point.to_owned(), T::zero())
    }

    pub fn min_distance(&self, point: &[T]) -> T {
        let distance = euclidean(&self.center, point);
        T::zero().max(distance - (self.radius)).powi(2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_sphere_min_distance() {
        let sphere1 = Sphere::new(vec![0., 0.], 10.);
        let point1 = vec![15., 0.];
        assert_eq!(sphere1.min_distance(&point1), 25.);
    }
}
