use num_traits::Float;
use num_traits::FromPrimitive;
use num_traits::Zero;
use std::ops::AddAssign;
use std::ops::DivAssign;
use std::ops::MulAssign;
use std::ops::SubAssign;

#[derive(Debug)]
pub struct Sphere<T, const dimension: usize> {
    center: [T; dimension],
    radius: T,
}

impl<T, const dimension: usize> Sphere<T, dimension>
where
    T: Float + Zero + FromPrimitive + AddAssign + SubAssign + DivAssign + MulAssign,
{
    pub fn new(center: [T; dimension], radius: T) -> Sphere<T, dimension> {
        Sphere { center, radius }
    }

    pub fn from_points(points: &Vec<[T; dimension]>) -> Sphere<T, dimension> {
        // todo
        Sphere::new([T::zero(); dimension], T::zero())
    }

    pub fn diameter(&self) -> T {
        self.radius + self.radius
    }

    pub fn distance2(&self, point: [T; dimension]) -> T {
        // 1. Calculate distance from center to point
        let mut distance = T::zero();
        for i in 0..dimension {
            distance += (self.center[i] - point[i]).powi(2);
        }
        // 2. Return 0 if the distance is less radius, otherwise distance - radius.
        T::zero().max(distance.sqrt() - self.radius)
    }

    pub fn contains(&self, point: [T; dimension]) -> bool {
        self.distance2(point) <= T::zero()
    }

    pub fn intersects(&self, sphere: Sphere<T, dimension>) -> bool {
        for i in 0..dimension {
            if sphere.center[i] + sphere.radius < self.center[i] - self.radius
                || self.center[i] + self.radius < sphere.center[i] - sphere.radius
            {
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
    pub fn test_sphere_diameter() {
        let sphere = Sphere::new([0., 0.], 5.);
        let expected = 10.;
        assert_eq!(expected, sphere.diameter());
    }

    #[test]
    pub fn test_distance_to_inside_point(){
        let sphere = Sphere::new([0., 0.], 10.);
        let point = [0.,5.];
        assert_eq!(sphere.distance2(point), 0.);
    }

    #[test]
    pub fn test_distance_to_outside_point(){
        let sphere = Sphere::new([0., 0.], 10.);
        let point = [0.,15.];
        assert_eq!(sphere.distance2(point), 5.);
    }

    #[test]
    pub fn test_sphere_contains_point() {
        let sphere = Sphere::new([0., 0.], 50.);
        let point = [10., 10.];
        assert!(sphere.contains(point));
    }

    #[test]
    pub fn test_sphere_doesnot_contain_point() {
        let sphere = Sphere::new([0., 0.], 5.);
        let point1 = [10., 10.];
        let point2 = [5.,5.];
        assert_eq!(sphere.contains(point1), false);
        assert_eq!(sphere.contains(point2), false);
    }

    #[test]
    pub fn test_sphere_intersects_sphere() {
        let sphere1 = Sphere::new([0., 0.], 10.);
        let sphere2 = Sphere::new([15., 15.], 10.);
        assert!(sphere1.intersects(sphere2));
    }

    #[test]
    pub fn test_sphere_doesnot_intersect_sphere() {
        let sphere1 = Sphere::new([0., 0.], 10.);
        let sphere2 = Sphere::new([15., 15.], 4.);
        assert_eq!(sphere1.intersects(sphere2), false);
    }

    #[test]
    pub fn test_sphere_intersects_its_clone() {
        let sphere1 = Sphere::new([0., 0.], 10.);
        let sphere2 = Sphere::new([0., 0.], 10.);
        assert!(sphere1.intersects(sphere2));
    }

    #[test]
    pub fn test_sphere_intersects_smaller_sphere() {
        let sphere1 = Sphere::new([0., 0.], 10.);
        let sphere2 = Sphere::new([10., 10.], 100.);
        assert!(sphere1.intersects(sphere2));
    }

    #[test]
    pub fn test_sphere_creation_from_points() {
        let mut points = Vec::new();
        points.push([100., 100.]);
        points.push([100., 150.]);
        points.push([250., 250.]);
        points.push([150., 300.]);
        let sphere = Sphere::from_points(&points);
        assert_eq!(sphere.center[0], 150.);
        assert_eq!(sphere.center[1], 200.);
    }
}
