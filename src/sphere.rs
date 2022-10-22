use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Mul;
use std::ops::MulAssign;
use std::ops::Sub;
use std::ops::SubAssign;

#[derive(Debug)]
pub struct Sphere<const dimension: usize, T> {
    center: [T; dimension],
    radius: T
}

impl<const dimension: usize, T> Sphere<dimension, T> where T: Copy + Add<Output = T> + AddAssign + Sub<Output = T> + SubAssign + Mul<Output = T> + MulAssign + Ord + Default {
    pub fn new(center: [T; dimension], radius: T) -> Sphere<dimension, T>{
        Sphere { center, radius }
    }

    pub fn diameter(&self) -> T {
        self.radius + self.radius
    }

    pub fn contains(&self, point: [T; dimension]) -> bool {
        for i in 0..dimension {
            if point[i] < self.center[i] - self.radius || self.center[i] + self.radius < point[i] {
                return false;
            }
        }
        true
    }

    pub fn intersects(&self, sphere: Sphere<dimension, T>) -> bool {
        for i in 0..dimension {
            if sphere.center[i] + sphere.radius < self.center[i] - self.radius || self.center[i] + self.radius < sphere.center[i] - sphere.radius {
                return false;
            }
        }
        true
    }
}

#[test]
pub fn test_sphere_diameter() {
    let sphere = Sphere::new([0,0], 5);
    let expected = 10;
    assert_eq!(expected, sphere.diameter());
}

#[test]
pub fn test_sphere_contains_point() {
    let sphere = Sphere::new([0,0], 50);
    let point = [10, 10];
    assert!(sphere.contains(point));
}

#[test]
pub fn test_sphere_doesnot_contain_point() {
    let sphere = Sphere::new([0,0], 5);
    let point = [10, 10];
    assert_eq!(sphere.contains(point), false);
}

#[test]
pub fn test_sphere_intersects_sphere() {
    let sphere1 = Sphere::new([0,0], 10);
    let sphere2 = Sphere::new([15,15], 10);
    assert!(sphere1.intersects(sphere2));
}

#[test]
pub fn test_sphere_doesnot_intersect_sphere() {
    let sphere1 = Sphere::new([0,0], 10);
    let sphere2 = Sphere::new([15,15], 4);
    assert_eq!(sphere1.intersects(sphere2), false);
}