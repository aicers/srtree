use crate::measure::distance::euclidean;
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

    pub fn from_point(point: &[T]) -> Rect<T> {
        Rect::new(point.to_owned(), point.to_owned())
    }

    pub fn min_distance(&self, point: &[T]) -> T {
        let mut closest_point = vec![T::infinity(); self.low.len()];
        for i in 0..self.low.len() {
            if point[i] < self.low[i] {
                closest_point[i] = self.low[i];
            } else if point[i] > self.high[i] {
                closest_point[i] = self.high[i];
            } else {
                closest_point[i] = point[i];
            }
        }
        euclidean(&closest_point, point)
    }

    pub fn farthest_point_to(&self, point: &[T]) -> Vec<T> {
        let mut result = self.low.clone();
        for i in 0..point.len() {
            if (self.high[i] - point[i]).abs() >= (self.low[i] - point[i]).abs() {
                result[i] = self.high[i];
            } else {
                result[i] = self.low[i];
            }
        }
        result
    }

    #[allow(dead_code)]
    pub fn min_max_distance(&self, point: &[T]) -> T {
        let min_max = self.farthest_point_to(point);
        let mut distance = euclidean(&min_max, point);
        for i in 0..self.low.len() {
            let mut current = min_max.clone();
            if current[i] == self.low[i] {
                current[i] = self.high[i];
            } else {
                current[i] = self.low[i];
            }
            let current_distance = euclidean(&current, point);
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
        assert_eq!(rec.min_distance(&vec![5., 0.]), 5.);
    }

    #[test]
    pub fn test_rect_farthest_point() {
        let rec = Rect::new(vec![5., 5.], vec![10., 10.]);
        assert_eq!(&rec.farthest_point_to(&vec![0., 0.]), &vec![10., 10.]);
        assert_eq!(&rec.farthest_point_to(&vec![15., 0.]), &vec![5., 10.]);
        assert_eq!(&rec.farthest_point_to(&vec![0., 15.]), &vec![10., 5.]);
        assert_eq!(&rec.farthest_point_to(&vec![15., 15.]), &vec![5., 5.]);
        assert_eq!(&rec.farthest_point_to(&vec![15., 5.]), &vec![5., 10.]);
    }

    #[test]
    pub fn test_rect_min_max_distance() {
        let rec = Rect::new(vec![5., 5.], vec![10., 10.]);
        assert_eq!(rec.min_max_distance(&vec![15., 5.]), (50.).sqrt());
    }
}
