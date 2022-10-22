/*
   A polytope is a geometric object with flat sides. It is a generalization in any number of dimensions of the three-dimensional polyhedron
   See more: https://en.wikipedia.org/wiki/Polytope

   In this library, a polytope specifies a region of the intersection of a bounding sphere and a bounding rectangle. 
*/

use crate::rect::Rect;
use crate::sphere::Sphere;

pub struct Polytope<const dimension: usize, T> {
    rect: Rect<dimension, T>,
    sphere: Sphere<dimension, T>
}