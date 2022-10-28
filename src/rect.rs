use crate::distance::Euclidean;
use num_traits::Float;
use num_traits::FromPrimitive;
use num_traits::Zero;
use std::ops::AddAssign;
use std::ops::DivAssign;
use std::ops::MulAssign;
use std::ops::SubAssign;

#[derive(Debug)]
pub struct Rect<T, const dimension: usize> {
    low: [T; dimension],
    high: [T; dimension],
}

impl<const dimension: usize, T> Rect<T, dimension>
where
    T: Float + Zero + FromPrimitive + AddAssign + SubAssign + DivAssign + MulAssign,
{
    pub fn new(low: [T; dimension], high: [T; dimension]) -> Rect<T, dimension> {
        Rect { low, high }
    }

    pub fn from_point(point: [T; dimension]) -> Rect<T, dimension> {
        Rect::new(point, point)
    }

    pub fn from_points(points: &Vec<[T; dimension]>) -> Rect<T, dimension> {
        let mut low = [T::max_value(); dimension];
        let mut high = [T::min_value(); dimension];

        points.iter().for_each(|point| {
            for i in 0..dimension {
                low[i] = low[i].min(point[i]);
                high[i] = high[i].max(point[i]);
            }
        });

        Rect { low, high }
    }

    pub fn volume(&self) -> T {
        if dimension == 0 {
            return T::zero();
        }
        let mut area = self.high[0] - self.low[0];
        for i in 1..dimension {
            area *= self.high[i] - self.low[i];
        }
        area
    }

    pub fn contains(&self, point: &[T; dimension]) -> bool {
        for i in 0..dimension {
            if point[i] < self.low[i] || self.high[i] < point[i] {
                return false;
            }
        }
        true
    }

    pub fn intersects(&self, rect: &Rect<T, dimension>) -> bool {
        for i in 0..dimension {
            if rect.high[i] < self.low[i] || self.high[i] < rect.low[i] {
                return false;
            }
        }
        true
    }

    pub fn farthest_distance2(&self, point: &[T; dimension]) -> T {
        let mut distance = T::zero();

        let mut farthest_vertex = [T::zero(); dimension];
        for i in 0..dimension {
            let h = (self.high[i] - point[i]).abs();
            let l = (self.low[i] - point[i]).abs();
            if h > l {
                farthest_vertex[i] = self.high[i];
            }else{
                farthest_vertex[i] = self.low[i];
            }
        }
        distance = Euclidean::distance(&farthest_vertex, point);
        distance
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_rect_area() {
        let rec1 = Rect::new([0., 0.], [10., 10.]);
        let expected = 100.;
        assert_eq!(expected, rec1.volume());
    }

    #[test]
    pub fn test_rect_area_from_point() {
        let rec1 = Rect::from_point([0., 0.]);
        let expected = 0.;
        assert_eq!(expected, rec1.volume());
    }

    #[test]
    pub fn test_rect_contains_point() {
        let rec = Rect::new([0., 0.], [10., 10.]);
        let point = [5., 5.];
        assert!(rec.contains(&point));
    }

    #[test]
    pub fn test_rect_doesnot_contain_point() {
        let rec = Rect::new([0., 0.], [10., 10.]);
        let point = [5., 15.];
        assert_eq!(rec.contains(&point), false)
    }

    #[test]
    pub fn test_rect_intersects_rect() {
        let rec1 = Rect::new([0., 0.], [10., 10.]);
        let rec2 = Rect::new([8., 8.], [15., 15.]);
        assert!(rec1.intersects(&rec2));
    }

    #[test]
    pub fn test_rect_doesnot_intersect_rect() {
        let rec1 = Rect::new([0., 0.], [10., 10.]);
        let rec2 = Rect::new([20., 20.], [30., 30.]);
        assert_eq!(rec1.intersects(&rec2), false);
    }

    #[test]
    pub fn test_rect_intersects_its_clone() {
        let rec1 = Rect::new([0., 0.], [10., 10.]);
        let rec2 = Rect::new([0., 0.], [10., 10.]);
        assert!(rec1.intersects(&rec2));
    }

    #[test]
    pub fn test_rect_intersects_smaller_rect() {
        let rec1 = Rect::new([0., 0.], [10., 10.]);
        let rec2 = Rect::new([2., 2.], [8., 8.]);
        assert!(rec1.intersects(&rec2));
    }

    #[test]
    pub fn test_rect_creation_from_points() {
        let points = vec![[0., 0.], [10., 10.], [15., 10.]];
        let rec = Rect::from_points(&points);
        assert_eq!(rec.low[0], 0.);
        assert_eq!(rec.high[0], 15.);
        assert_eq!(rec.low[1], 0.);
        assert_eq!(rec.high[1], 10.);
    }

    #[test]
    pub fn test_rect_farthest_distance_to_point(){
        let points = vec![[0.,0.],[10.,10.]];
        let rec = Rect::from_points(&points);
        let point = [10.,10.];
        assert_eq!(rec.farthest_distance2(&point), 200.0.sqrt());
    }
}
