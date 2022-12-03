use crate::measure::distance::euclidean;
use crate::measure::mean;
use crate::measure::variance;
use crate::node::Node;
use ordered_float::Float;
use std::{
    fmt::Debug,
    ops::{AddAssign, DivAssign, MulAssign, SubAssign},
};

use super::{rect::Rect, sphere::Sphere};

pub fn reshape<T>(node: &mut Node<T>)
where
    T: Debug + Copy + Float + AddAssign + SubAssign + MulAssign + DivAssign,
{
    let centroid = mean::calculate(node, 0, node.immed_children());
    let mut ds = T::zero(); // farthest point to child spheres
    let mut dr = T::zero(); // farthest point to child rectangles

    let mut low = centroid.clone();
    let mut high = centroid.clone();
    let mut total_children = 0;
    if node.is_leaf() {
        node.points().iter().for_each(|point| {
            for i in 0..node.dimension() {
                low[i] = low[i].min(point[i]);
                high[i] = high[i].max(point[i]);
            }
            ds = ds.max(euclidean(&centroid, point));
            dr = ds;
        });
        total_children = node.points().len();
    } else {
        node.nodes().iter().for_each(|child| {
            for i in 0..child.dimension() {
                low[i] = low[i].min(child.get_rect().low[i]);
                high[i] = high[i].max(child.get_rect().high[i]);
            }
            ds = ds
                .max(euclidean(&centroid, &child.get_sphere().center) + child.get_sphere().radius);
            dr = dr.max(euclidean(
                &centroid,
                &child.get_rect().farthest_point_to(&centroid),
            ));
            total_children += child.get_total_children();
        });
    }
    let rect = Rect::new(low, high);
    node.set_rect(rect);

    let radius = ds.min(dr);
    node.set_sphere(Sphere::new(centroid, radius));

    let variance = variance::calculate(node, 0, node.immed_children());
    node.set_variance(variance);

    node.set_total_children(total_children);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_reshape_leaf_node() {
        let origin = vec![0., 0.];
        let mut leaf = Node::new_leaf(&origin, 5);
        for i in 0..5 {
            leaf.points_mut().push(vec![0., i as f64]);
        }
        reshape(&mut leaf);

        assert_eq!(&leaf.get_sphere().center, &vec![0., 2.]);
        assert_eq!(&leaf.get_rect().low, &vec![0., 0.]);
        assert_eq!(&leaf.get_rect().high, &vec![0., 4.]);

        leaf.points_mut().pop();
        leaf.points_mut().pop();

        reshape(&mut leaf);
        assert_eq!(&leaf.get_sphere().center, &vec![0., 1.]);
        assert_eq!(&leaf.get_rect().low, &vec![0., 0.]);
        assert_eq!(&leaf.get_rect().high, &vec![0., 2.]);
    }

    #[test]
    pub fn test_reshape_leaf_node_radius() {
        let origin = vec![0., 0.];
        let mut leaf = Node::new_leaf(&origin, 5);
        leaf.points_mut().push(vec![100., 100.]);
        leaf.points_mut().push(vec![100., 150.]);
        leaf.points_mut().push(vec![250., 250.]);
        leaf.points_mut().push(vec![150., 300.]);
        reshape(&mut leaf);
        assert_eq!(leaf.get_sphere().radius, 111.80339887498948);
    }
}
