use num_traits::Float;
use num_traits::FromPrimitive;
use num_traits::Zero;
use std::ops::AddAssign;
use std::ops::DivAssign;
use std::ops::MulAssign;
use std::ops::SubAssign;

use crate::distance::distance;


#[allow(dead_code)]
#[derive(Debug)]
pub struct Sphere<T> {
    pub center: Vec<T>,
    pub radius: T,
}

#[allow(dead_code)]
impl<T> Sphere<T>
where
    T: Float + Zero + FromPrimitive + AddAssign + SubAssign + DivAssign + MulAssign,
{
    pub fn new(center: Vec<T>, radius: T) -> Sphere<T> {
        Sphere { center, radius }
    }

    pub fn from_point(point: &[T]) -> Sphere<T> {
        Sphere::new(point.to_owned(), T::zero())
    }

    pub fn intersects(&self, sphere: &Sphere<T>) -> bool {
        let distance = distance(&self.center, &sphere.center);
        distance - (self.radius + sphere.radius) <= T::zero()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_from_point() {
        let sphere = Sphere::from_point(&vec![1., 1.]);
        assert_eq!(sphere.radius, 0.);
    }

    #[test]
    pub fn test_sphere_intersects_sphere() {
        let sphere1 = Sphere::new(vec![0., 0.], 10.);
        let sphere2 = Sphere::new(vec![15., 15.], 15.);
        assert!(sphere1.intersects(&sphere2));
    }

    #[test]
    pub fn test_sphere_doesnot_intersect_sphere() {
        let sphere1 = Sphere::new(vec![450., 150.], 50.);
        let sphere2 = Sphere::new(vec![530., 220.], 50.);
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
