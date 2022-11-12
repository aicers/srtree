use crate::algorithm::distance::euclidean;
use ordered_float::Float;
use std::ops::{AddAssign, DivAssign, MulAssign, SubAssign};

#[allow(dead_code)]
#[derive(Debug)]
pub struct Sphere<T> {
    pub center: Vec<T>,
    pub radius: T,
}

#[allow(dead_code)]
impl<T> Sphere<T>
where
    T: Float + AddAssign + SubAssign + MulAssign + DivAssign,
{
    pub fn new(center: Vec<T>, radius: T) -> Sphere<T> {
        Sphere { center, radius }
    }

    pub fn from_point(point: &[T]) -> Sphere<T> {
        Sphere::new(point.to_owned(), T::zero())
    }

    pub fn distance2(&self, point: &Vec<T>) -> T {
        let distance = euclidean(&self.center, point);
        T::zero().max(distance - (self.radius))
    }

    pub fn intersects_point(&self, point: &Vec<T>) -> bool {
        self.distance2(point) <= T::zero()
    }

    pub fn intersects(&self, sphere: &Sphere<T>) -> bool {
        self.distance2(&sphere.center) - (self.radius + sphere.radius) <= T::zero()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_sphere_intersects_point() {
        let sphere1 = Sphere::new(vec![0., 0.], 10.);
        let point1 = vec![5., 5.];
        assert!(sphere1.intersects_point(&point1));
    }

    #[test]
    pub fn test_sphere_doesnot_intersect_point() {
        let sphere1 = Sphere::new(vec![0., 0.], 10.);
        let point2 = vec![15., 15.];
        assert!(!sphere1.intersects_point(&point2));
    }

    #[test]
    pub fn test_sphere_intersects_sphere() {
        let sphere1 = Sphere::new(vec![0., 0.], 10.);
        let sphere2 = Sphere::new(vec![15., 15.], 15.);
        assert!(sphere1.intersects(&sphere2));
    }

    #[test]
    pub fn test_sphere_doesnot_intersect_sphere() {
        let sphere1 = Sphere::new(vec![0., 0.], 5.);
        let sphere2 = Sphere::new(vec![20., 20.], 5.);
        assert_eq!(sphere1.intersects(&sphere2), false);
    }

    #[test]
    pub fn test_sphere_intersects_its_clone() {
        let sphere1 = Sphere::new(vec![0., 0.], 10.);
        let sphere2 = Sphere::new(vec![0., 0.], 10.);
        assert!(sphere1.intersects(&sphere2));
    }

    #[test]
    pub fn test_sphere_intersects_smaller_sphere() {
        let sphere1 = Sphere::new(vec![0., 0.], 10.);
        let sphere2 = Sphere::new(vec![10., 10.], 100.);
        assert!(sphere1.intersects(&sphere2));
    }
}
