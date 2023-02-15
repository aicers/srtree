use crate::{measure::distance::euclidean_squared, shape::point::Point};
use ordered_float::Float;
use std::{
    fmt::Debug,
    ops::{AddAssign, DivAssign, MulAssign, SubAssign},
};

#[derive(Debug)]
pub struct Rect<T> {
    pub low: Vec<T>,
    pub high: Vec<T>,
}

impl<T> Rect<T>
where
    T: Debug + Copy + Float + AddAssign + SubAssign + MulAssign + DivAssign,
{
    pub fn new(low: Vec<T>, high: Vec<T>) -> Rect<T> {
        Rect { low, high }
    }

    pub fn from_point(point: &Point<T>) -> Rect<T> {
        Rect::new(point.coords().clone(), point.coords().clone())
    }

    pub fn min_distance(&self, point: &Point<T>) -> T {
        let mut closest_point = Point::with_coords(vec![T::infinity(); self.low.len()]);
        for i in 0..self.low.len() {
            if point.coord_at(i) < self.low[i] {
                closest_point.set_coord_at(i, self.low[i]);
            } else if point.coord_at(i) > self.high[i] {
                closest_point.set_coord_at(i, self.high[i]);
            } else {
                closest_point.set_coord_at(i, point.coord_at(i));
            }
        }
        euclidean_squared(&closest_point, point)
    }

    pub fn farthest_point_to(&self, point: &Point<T>) -> Point<T> {
        let mut result = Point::with_coords(self.low.clone());
        for i in 0..point.dimension() {
            if (self.high[i] - point.coord_at(i)).abs() >= (self.low[i] - point.coord_at(i)).abs() {
                result.set_coord_at(i, self.high[i]);
            } else {
                result.set_coord_at(i, self.low[i]);
            }
        }
        result
    }

    #[allow(dead_code)]
    pub fn min_max_distance(&self, point: &Point<T>) -> T {
        let min_max = self.farthest_point_to(point);
        let mut distance = euclidean_squared(&min_max, point);
        for i in 0..self.low.len() {
            let mut current = min_max.clone();
            if current.coord_at(i) == self.low[i] {
                current.set_coord_at(i, self.high[i]);
            } else {
                current.set_coord_at(i, self.low[i]);
            }
            let current_distance = euclidean_squared(&current, point);
            if current_distance < distance {
                distance = current_distance;
            }
        }
        distance
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_rect_min_distance() {
        let rec = Rect::new(vec![5., 5.], vec![10., 10.]);
        assert_eq!(rec.min_distance(&Point::with_coords(vec![5., 0.])), 25.);
    }

    #[test]
    pub fn test_rect_farthest_point() {
        let rec = Rect::new(vec![5., 5.], vec![10., 10.]);
        assert_eq!(
            rec.farthest_point_to(&Point::with_coords(vec![0., 0.]))
                .coords(),
            &vec![10., 10.]
        );
        assert_eq!(
            rec.farthest_point_to(&Point::with_coords(vec![15., 0.]))
                .coords(),
            &vec![5., 10.]
        );
        assert_eq!(
            rec.farthest_point_to(&Point::with_coords(vec![0., 15.]))
                .coords(),
            &vec![10., 5.]
        );
        assert_eq!(
            rec.farthest_point_to(&Point::with_coords(vec![15., 15.]))
                .coords(),
            &vec![5., 5.]
        );
        assert_eq!(
            rec.farthest_point_to(&Point::with_coords(vec![15., 5.]))
                .coords(),
            &vec![5., 10.]
        );
    }

    #[test]
    pub fn test_rect_min_max_distance() {
        let rec = Rect::new(vec![5., 5.], vec![10., 10.]);
        assert_eq!(
            rec.min_max_distance(&Point::with_coords(vec![15., 5.])),
            50.
        );
    }
}
