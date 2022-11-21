use crate::algorithm::distance::euclidean;
use ordered_float::Float;
use std::ops::{AddAssign, DivAssign, MulAssign, SubAssign};

#[allow(dead_code)]
#[derive(Debug)]
pub struct Sphere<T> {
    center: Vec<T>,
    radius: T,
}

#[allow(dead_code)]
impl<T> Sphere<T>
where
    T: Float + AddAssign + SubAssign + MulAssign + DivAssign,
{
    pub fn new(center: Vec<T>, radius: T) -> Sphere<T> {
        Sphere { center, radius }
    }

    pub fn reshape_with(&mut self, points: &Vec<Vec<T>>){
        if points.is_empty() {
            return;
        }
        if points[0].len() != self.center.len() {
            panic!("Trying to reshape Sphere with different dimensions");
        }

        let number_of_points = T::from(points.len()).unwrap();
        let mut new_centroid: Vec<T> = vec![T::zero(); points[0].len()];
        points.iter().for_each(|point| {
            for i in 0..point.len() {
                new_centroid[i] += point[i];
            }
        });
        for i in 0..new_centroid.len() {
            new_centroid[i] /= number_of_points;
        }

        self.center = new_centroid;
    }

    pub fn from_point(point: &[T]) -> Sphere<T> {
        Sphere::new(point.to_owned(), T::zero())
    }

    pub fn centroid(&self) -> &Vec<T> {
        &self.center
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
