use super::{point::Point, rect::Rect, sphere::Sphere};
use crate::measure::mean;
use crate::node::Node;
use ordered_float::{Float, OrderedFloat};
use std::{
    fmt::Debug,
    ops::{AddAssign, DivAssign, MulAssign, SubAssign},
};

pub fn reshape<T>(node: &mut Node<T>)
where
    T: Debug + Copy + Float + AddAssign + SubAssign + MulAssign + DivAssign,
{
    let centroid = Point::with_coords(mean::calculate(node, 0, node.immed_children()));
    let mut max_distance = T::zero();

    let mut low = centroid.coords.clone();
    let mut high = centroid.coords.clone();
    if node.is_leaf() {
        node.points_mut().iter_mut().for_each(|point| {
            for i in 0..point.dimension() {
                low[i] = low[i].min(point.coords[i]);
                high[i] = high[i].max(point.coords[i]);
            }
            let distance_to_point = centroid.distance(point);
            point.radius = distance_to_point; // set radius of point to distance to centroid
            max_distance = max_distance.max(distance_to_point);
        });
        node.points_mut()
            .sort_by_key(|point| -OrderedFloat(point.radius)); // sort by radius in descending order
    } else {
        node.nodes().iter().for_each(|child| {
            for i in 0..child.dimension() {
                low[i] = low[i].min(child.rect.low[i]);
                high[i] = high[i].max(child.rect.high[i]);
            }
            let distance = child.max_distance(&centroid);
            max_distance = max_distance.max(distance);
        });
    }
    node.rect = Rect::new(low, high);;
    node.sphere = Sphere::new(centroid, max_distance);
}

#[cfg(test)]
mod tests {
    use crate::shape::point::Point;

    use super::*;

    #[test]
    pub fn test_reshape_leaf_node() {
        let origin = Point::with_coords(vec![0., 0.]);
        let mut leaf = Node::new_leaf(&origin, 5);
        for i in 0..5 {
            leaf.points_mut()
                .push(Point::with_coords(vec![0., i as f64]));
        }
        reshape(&mut leaf);

        assert_eq!(leaf.sphere.center.coords, vec![0., 2.]);
        assert_eq!(&leaf.rect.low, &vec![0., 0.]);
        assert_eq!(&leaf.rect.high, &vec![0., 4.]);
    }

    #[test]
    pub fn test_reshape_leaf_node_radius() {
        let origin = Point::with_coords(vec![0., 0.]);
        let mut leaf = Node::new_leaf(&origin, 5);
        leaf.points_mut().push(Point::with_coords(vec![100., 100.]));
        leaf.points_mut().push(Point::with_coords(vec![100., 150.]));
        leaf.points_mut().push(Point::with_coords(vec![250., 250.]));
        leaf.points_mut().push(Point::with_coords(vec![150., 300.]));
        reshape(&mut leaf);
        assert_eq!(leaf.sphere.radius, 111.80339887498948);
    }

    #[test]
    pub fn test_reshape_leaf_points_radius() {
        let origin = Point::with_coords(vec![0., 0.]);
        let mut leaf = Node::new_leaf(&origin, 5);
        leaf.points_mut().push(Point::with_coords(vec![0., 0.]));
        leaf.points_mut().push(Point::with_coords(vec![1., 1.]));
        leaf.points_mut().push(Point::with_coords(vec![2., 2.]));
        leaf.points_mut().push(Point::with_coords(vec![3., 3.]));
        leaf.points_mut().push(Point::with_coords(vec![4., 4.]));
        reshape(&mut leaf);
        // points are sorted by radius in descending order
        assert_eq!(leaf.points()[0].radius, 2.8284271247461903);
        assert_eq!(leaf.points()[1].radius, 2.8284271247461903);
        assert_eq!(leaf.points()[2].radius, 1.4142135623730951);
        assert_eq!(leaf.points()[3].radius, 1.4142135623730951);
        assert_eq!(leaf.points()[4].radius, 0.0);
    }
}
