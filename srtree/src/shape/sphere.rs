use super::point::Point;
use crate::measure::distance::euclidean;
use ordered_float::Float;
use std::{
    fmt::Debug,
    ops::{AddAssign, DivAssign, MulAssign, SubAssign},
};

#[derive(Debug)]
pub struct Sphere<T> {
    pub center: Point<T>,
    pub radius: T,
}

impl<T> Sphere<T>
where
    T: Debug + Copy + Float + AddAssign + SubAssign + MulAssign + DivAssign,
{
    pub fn new(center: Point<T>, radius: T) -> Sphere<T> {
        Sphere { center, radius }
    }

    pub fn from_point(point: &Point<T>) -> Sphere<T> {
        Sphere::new(point.clone(), T::zero())
    }

    pub fn min_distance(&self, point: &Point<T>) -> T {
        let distance = euclidean(&self.center, point);
        T::zero().max(distance - (self.radius)).powi(2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_sphere_min_distance() {
        let sphere1 = Sphere::new(Point::with_coords(vec![0., 0.]), 10.);
        let point1 = Point::with_coords(vec![15., 0.]);
        assert_eq!(sphere1.min_distance(&point1), 25.);
    }
}
