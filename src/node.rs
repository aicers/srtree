use num_traits::Float;
use num_traits::FromPrimitive;
use num_traits::Zero;
use std::ops::AddAssign;
use std::ops::DivAssign;
use std::ops::MulAssign;
use std::ops::SubAssign;
use crate::rect::Rect;
use crate::sphere::Sphere;


pub enum Data<T, const dimension: usize> {
    Point([T; dimension]), // point and additional data
    Nodes(Vec<Node<T, dimension>>) 
}

pub struct Node<T, const dimension: usize>{
    rect: Rect<T, dimension>,
    sphere: Sphere<T, dimension>,
    data: Data<T, dimension>,
    number_of_points: usize
}

impl<T, const dimension: usize> Node<T, dimension>
where
    T: Float + Zero + FromPrimitive + AddAssign + SubAssign + DivAssign + MulAssign,
{
    pub fn from_point(point: [T; dimension]) -> Node<T, dimension> {
        Node { 
            rect: Rect::from_point(point), 
            sphere: Sphere::from_point(point), 
            data: Data::Point(point), 
            number_of_points: 1 
        }
    }

    pub fn from_points(points: &Vec<[T; dimension]>) -> Node<T, dimension>{
        let rect = Rect::from_points(points);
        let mut sphere = Sphere::from_points(points);
        sphere.radius = sphere.radius.min(rect.farthest_distance2(&sphere.center));

        Node { 
            rect: rect, 
            sphere: sphere, 
            data: Data::Nodes(Vec::new()), 
            number_of_points: points.len()
        }
    }

    pub fn contains(&self, point: &[T; dimension]) -> bool {
        self.rect.contains(point) && self.sphere.contains(point)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_intersection(){
        let mut points = Vec::new();
        points.push([0.,0.]);
        points.push([0.,1.]);
        points.push([0.,4.]);
        points.push([1.,0.]);
        points.push([1.,1.]);
        let node = Node::from_points(&points);
        assert_eq!(node.sphere.center, [0.4, 1.2]);
        assert!(node.sphere.radius > 2.82 && node.sphere.radius < 2.83); // 2.82842712474619
        assert_eq!(node.contains(&[1.,4.]), false);
    }
}