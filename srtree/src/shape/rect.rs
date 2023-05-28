use crate::shape::point::Point;
use ordered_float::Float;

pub struct Rect<T> {
    pub low: Vec<T>,
    pub high: Vec<T>,
}

impl<T> Rect<T>
where
    T: Float + Send + Sync,
{
    pub fn new(low: Vec<T>, high: Vec<T>) -> Rect<T> {
        Rect { low, high }
    }

    pub fn from_point(point: &Point<T>) -> Rect<T> {
        Rect::new(point.coords.clone(), point.coords.clone())
    }

    pub fn closest_point_to(&self, point: &Point<T>) -> Point<T> {
        let mut closest_point = Point::with_coords(vec![T::infinity(); point.dimension()]);
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

    pub fn farthest_point_to(&self, point: &Point<T>) -> Point<T> {
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_rect_closest_point() {
        let rec = Rect::new(vec![5., 5.], vec![10., 10.]);
        assert_eq!(
            rec.closest_point_to(&Point::with_coords(vec![0., 0.]))
                .coords,
            [5., 5.]
        );
        assert_eq!(
            rec.closest_point_to(&Point::with_coords(vec![15., 0.]))
                .coords,
            [10., 5.]
        );
        assert_eq!(
            rec.closest_point_to(&Point::with_coords(vec![0., 15.]))
                .coords,
            [5., 10.]
        );
        assert_eq!(
            rec.closest_point_to(&Point::with_coords(vec![15., 15.]))
                .coords,
            [10., 10.]
        );
        assert_eq!(
            rec.closest_point_to(&Point::with_coords(vec![7., 7.]))
                .coords,
            [7., 7.]
        );
    }

    #[test]
    pub fn test_rect_farthest_point() {
        let rec = Rect::new(vec![5., 5.], vec![10., 10.]);
        assert_eq!(
            rec.farthest_point_to(&Point::with_coords(vec![0., 0.]))
                .coords,
            [10., 10.]
        );
        assert_eq!(
            rec.farthest_point_to(&Point::with_coords(vec![15., 0.]))
                .coords,
            [5., 10.]
        );
        assert_eq!(
            rec.farthest_point_to(&Point::with_coords(vec![0., 15.]))
                .coords,
            [10., 5.]
        );
        assert_eq!(
            rec.farthest_point_to(&Point::with_coords(vec![15., 15.]))
                .coords,
            [5., 5.]
        );
        assert_eq!(
            rec.farthest_point_to(&Point::with_coords(vec![15., 5.]))
                .coords,
            [5., 10.]
        );
    }
}
