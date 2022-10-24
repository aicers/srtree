/*
   A polytope is a geometric object with flat sides. It is a generalization in any number of dimensions of the three-dimensional polyhedron
   See more: https://en.wikipedia.org/wiki/Polytope

   In this library, a polytope specifies a region of the intersection of a bounding sphere and a bounding rectangle.
*/

use crate::rect::Rect;
use crate::sphere::Sphere;
use num_traits::Float;
use num_traits::FromPrimitive;
use num_traits::Zero;
use std::ops::AddAssign;
use std::ops::DivAssign;
use std::ops::MulAssign;
use std::ops::SubAssign;

pub struct Polytope<T, const dimension: usize> {
    rect: Rect<T, dimension>,
    sphere: Sphere<T, dimension>,
}

impl<const dimension: usize, T> Polytope<T, dimension>
where
    T: Float + Zero + FromPrimitive + AddAssign + SubAssign + DivAssign + MulAssign,
{
    pub fn from_points(points: Vec<[T; dimension]>) -> Polytope<T, dimension> {
        Polytope {
            rect: Rect::from_points(&points),
            sphere: Sphere::from_points(&points),
        }
    }

    pub fn contains(&self, point: [T; dimension]) -> bool {
        self.rect.contains(point) && self.sphere.contains(point)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_polytope_creation_with_its_points() {
        let mut points = Vec::new();
        points.push([100., 100.]);
        points.push([100., 150.]);
        points.push([250., 250.]);
        points.push([150., 300.]);
        let polytope = Polytope::from_points(points);
        assert!(polytope.contains([100., 100.]));
        assert!(polytope.contains([100., 150.]));
        assert!(polytope.contains([250., 250.]));
        assert!(polytope.contains([150., 300.]));
        assert!(polytope.contains([150., 200.])); // sphere center
    }

    pub fn test_polytope_creation_with_outside_points() {
        let mut points = Vec::new();
        points.push([100., 100.]);
        points.push([100., 150.]);
        points.push([250., 250.]);
        points.push([150., 300.]);
        let polytope = Polytope::from_points(points);
        assert_eq!(polytope.contains([1000., 1000.]), false);
    }

    #[test]
    pub fn test_polytope_creation_with_rect_edge_points() {
        let mut points = Vec::new();
        points.push([100., 100.]);
        points.push([100., 150.]);
        points.push([250., 250.]);
        points.push([150., 300.]);
        let polytope = Polytope::from_points(points);

        // these points lie within the polytope rect but not inside of its sphere:
        assert_eq!(polytope.contains([250., 300.]), false);
        assert_eq!(polytope.contains([100., 300.]), false);
    }

    #[test]
    pub fn test_polytope_creation_with_sphere_edge_points() {
        let mut points = Vec::new();
        points.push([100., 100.]);
        points.push([100., 150.]);
        points.push([250., 250.]);
        points.push([150., 300.]);
        let polytope = Polytope::from_points(points);

        // these points lie within the polytope sphere but not inside of its rect:
        assert_eq!(polytope.contains([99., 200.]), false);
        assert_eq!(polytope.contains([150., 301.]), false);
    }
}
