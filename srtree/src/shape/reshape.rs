use super::{point::Point, rect::Rect, sphere::Sphere};
use crate::measure::distance::euclidean;
use crate::measure::mean;
use crate::node::Node;
use ordered_float::Float;
use std::{
    fmt::Debug,
    ops::{AddAssign, DivAssign, MulAssign, SubAssign},
};

pub fn reshape<T>(node: &mut Node<T>)
where
    T: Debug + Copy + Float + AddAssign + SubAssign + MulAssign + DivAssign,
{
    let centroid = Point::with_coords(mean::calculate(node, 0, node.immed_children()));
    let mut ds = T::zero(); // farthest distance to child spheres
    let mut dr = T::zero(); // farthest distance to child rectangles

    let mut low = centroid.coords().clone();
    let mut high = centroid.coords().clone();
    if node.is_leaf() {
        node.points().iter().for_each(|point| {
            for i in 0..node.dimension() {
                low[i] = low[i].min(point.coord_at(i));
                high[i] = high[i].max(point.coord_at(i));
            }
            ds = ds.max(euclidean(&centroid, point));
            dr = ds;
        });
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
        });
    }
    let rect = Rect::new(low, high);
    node.set_rect(rect);

    let radius = ds.min(dr);
    node.set_sphere(Sphere::new(centroid, radius));
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

        assert_eq!(leaf.get_sphere().center.coords(), &vec![0., 2.]);
        assert_eq!(&leaf.get_rect().low, &vec![0., 0.]);
        assert_eq!(&leaf.get_rect().high, &vec![0., 4.]);

        leaf.points_mut().pop();
        leaf.points_mut().pop();

        reshape(&mut leaf);
        assert_eq!(leaf.get_sphere().center.coords(), &vec![0., 1.]);
        assert_eq!(&leaf.get_rect().low, &vec![0., 0.]);
        assert_eq!(&leaf.get_rect().high, &vec![0., 2.]);
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
        assert_eq!(leaf.get_sphere().radius, 111.80339887498948);
    }
}
