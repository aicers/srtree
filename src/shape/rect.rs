use ordered_float::Float;
use std::ops::{AddAssign, DivAssign, MulAssign, SubAssign};

#[allow(dead_code)]
#[derive(Debug)]
pub struct Rect<T> {
    low: Vec<T>,
    high: Vec<T>,
}

impl<T> Rect<T>
where
    T: Float + AddAssign + SubAssign + MulAssign + DivAssign,
{
    pub fn new(low: Vec<T>, high: Vec<T>) -> Option<Rect<T>> {
        if low.len() != high.len() {
            return None;
        }
        Some(Rect { low, high })
    }

    pub fn from_point(point: &[T]) -> Option<Rect<T>> {
        Rect::new(point.to_owned(), point.to_owned())
    }

    pub fn reshape_with(&mut self, points: &Vec<Vec<T>>){
        if points.is_empty() {
            return;
        }
        if points[0].len() != self.low.len() {
            panic!("Trying to reshape Rect with different dimensions");
        }

        self.low = points[0].clone();
        self.high = points[0].clone();

        points.iter().for_each(|point| {
            for j in 0..point.len() {
                self.low[j] = self.low[j].min(point[j]);
                self.high[j] = self.high[j].max(point[j]);
            }
        });
    }

    pub fn intersects_point(&self, point: &Vec<T>) -> bool {
        if self.low.len() != point.len() {
            return false;
        }
        for (i, d) in point.iter().enumerate() {
            if point[i] < self.low[i] || self.high[i] < point[i] {
                return false;
            }
        }
        true
    }

    pub fn intersects(&self, rec: &Rect<T>) -> bool {
        if self.low.len() != rec.low.len() {
            return false;
        }
        for i in 0..self.low.len() {
            if rec.high[i] < self.low[i] || self.high[i] < rec.low[i] {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_new_rect() {
        let rec1 = Rect::new(vec![1., 2., 3.], vec![1., 2.]);
        let rec2 = Rect::new(vec![1., 2.], vec![1., 2.]);
        assert!(rec1.is_none() && rec2.is_some());
    }

    #[test]
    pub fn test_reshape_rect(){
        let origin = vec![0., 0.];
        let mut rect = Rect::from_point(&origin).unwrap();

        let points = vec![vec![1., 18.], vec![10., 12.]];
        rect.reshape_with(&points);

        assert_eq!(rect.low, vec![1., 12.]);
        assert_eq!(rect.high, vec![10., 18.]);
    }

    #[test]
    pub fn test_intersects_point() {
        let rec = Rect::new(vec![0., 0.], vec![10., 10.]).unwrap();
        let point2 = &vec![5., 5.];
        assert!(rec.intersects_point(&point2));
    }

    #[test]
    pub fn test_doesnot_intersect_point() {
        let rec = Rect::new(vec![0., 0.], vec![10., 10.]).unwrap();
        let point1 = vec![11., 0.];
        assert_eq!(rec.intersects_point(&point1), false);
    }

    #[test]
    pub fn test_intersects_rect() {
        let rec = Rect::new(vec![0., 0.], vec![10., 10.]).unwrap();
        let rec2 = Rect::new(vec![5., 5.], vec![15., 15.]).unwrap();
        assert!(rec.intersects(&rec2));
    }

    #[test]
    pub fn test_doesnot_intersect_rect() {
        let rec = Rect::new(vec![0., 0.], vec![10., 10.]).unwrap();
        let rec2 = Rect::new(vec![15., 0.], vec![20., 10.]).unwrap();
        assert_eq!(rec.intersects(&rec2), false);
    }
}
