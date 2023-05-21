use crate::shape::point::Point;
use ordered_float::Float;
use std::fmt::Debug;

#[derive(Debug)]
pub struct Rect<T> {
    pub low: Vec<T>,
    pub high: Vec<T>,
}

impl<T> Rect<T>
where
    T: Debug + Copy + Float + Send + Sync,
{
    pub fn new(low: Vec<T>, high: Vec<T>) -> Rect<T> {
        Rect { low, high }
    }

    pub fn from_point(point: &Point<T>) -> Rect<T> {
        Rect::new(point.coords.clone(), point.coords.clone())
    }

    fn closest_point_to(&self, point: &Point<T>) -> Point<T> {
        let mut closest_point = Point::with_coords(vec![T::infinity(); self.low.len()]);
        for i in 0..self.low.len() {
            if point.coords[i] < self.low[i] {
                closest_point.coords[i] = self.low[i];
            } else if point.coords[i] > self.high[i] {
                closest_point.coords[i] = self.high[i];
            } else {
                closest_point.coords[i] = point.coords[i];
            }
        }
        closest_point
    }

    pub fn min_distance(&self, point: &Point<T>) -> T {
        let closest_point = self.closest_point_to(point);
        point.distance(&closest_point)
    }

    fn farthest_point_to(&self, point: &Point<T>) -> Point<T> {
        let mut result = Point::with_coords(self.low.clone());
        for i in 0..point.dimension() {
            if (self.high[i] - point.coords[i]).abs() >= (self.low[i] - point.coords[i]).abs() {
                result.coords[i] = self.high[i];
            } else {
                result.coords[i] = self.low[i];
            }
        }
        result
    }

    pub fn max_distance(&self, point: &Point<T>) -> T {
        let farthest_point = self.farthest_point_to(point);
        point.distance(&farthest_point)
    }

    #[allow(dead_code)]
    pub fn min_max_distance(&self, point: &Point<T>) -> T {
        let min_max = self.farthest_point_to(point);
        let mut distance = min_max.distance_squared(point);
        for i in 0..self.low.len() {
            let mut current = min_max.clone();
            if current.coords[i] == self.low[i] {
                current.coords[i] = self.high[i];
            } else {
                current.coords[i] = self.low[i];
            }
            let current_distance = point.distance_squared(&current);
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
        assert_eq!(rec.min_distance(&Point::with_coords(vec![5., 0.])), 5.);
        assert_eq!(rec.min_distance(&Point::with_coords(vec![7., 7.])), 0.);
    }

    #[test]
    pub fn test_rect_farthest_point() {
        let rec = Rect::new(vec![5., 5.], vec![10., 10.]);
        assert_eq!(
            rec.farthest_point_to(&Point::with_coords(vec![0., 0.]))
                .coords,
            vec![10., 10.]
        );
        assert_eq!(
            rec.farthest_point_to(&Point::with_coords(vec![15., 0.]))
                .coords,
            vec![5., 10.]
        );
        assert_eq!(
            rec.farthest_point_to(&Point::with_coords(vec![0., 15.]))
                .coords,
            vec![10., 5.]
        );
        assert_eq!(
            rec.farthest_point_to(&Point::with_coords(vec![15., 15.]))
                .coords,
            vec![5., 5.]
        );
        assert_eq!(
            rec.farthest_point_to(&Point::with_coords(vec![15., 5.]))
                .coords,
            vec![5., 10.]
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
