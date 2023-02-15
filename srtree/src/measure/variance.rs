use super::mean;
use crate::node::Node;
use ordered_float::Float;
use std::{
    fmt::Debug,
    ops::{AddAssign, DivAssign, MulAssign, SubAssign},
};

pub fn calculate<T>(node: &Node<T>, from: usize, end: usize) -> Vec<T>
where
    T: Debug + Float + AddAssign + SubAssign + MulAssign + DivAssign,
{
    if node.immed_children() == 0 || node.immed_children() < end || from >= end {
        return Vec::new();
    }

    // 1. Calculate mean (mean of entries)
    let mean = mean::calculate(node, from, end);

    // 2. Calculate variance w.r.t. the mean
    let mut number_of_entries = T::zero();
    let mut variance = vec![T::zero(); mean.len()];
    for child_index in from..end {
        let child_number_of_entries =
            T::from(node.child_immed_children(child_index)).unwrap_or_else(T::one);
        for axis_index in 0..variance.len() {
            variance[axis_index] +=
                (node.child_centroid(child_index).coord_at(axis_index) - mean[axis_index]).powi(2)
                    * child_number_of_entries;
            if !node.is_leaf() {
                variance[axis_index] +=
                    child_number_of_entries * node.child_variance(child_index)[axis_index];
            }
        }
        number_of_entries += child_number_of_entries;
    }
    for var in &mut variance {
        *var /= number_of_entries;
    }
    variance
}

#[cfg(test)]
mod tests {
    use crate::shape::point::Point;

    use super::*;

    pub fn get_test_points() -> Vec<Vec<f64>> {
        let mut points = Vec::new();
        points.push(vec![0., 0.]);
        points.push(vec![0., 1.]);
        points.push(vec![0., 2.]);
        points.push(vec![0., 8.]);
        points.push(vec![0., 9.]);
        points
    }

    #[test]
    pub fn test_variance_calculation() {
        let origin = Point::with_coords(vec![0., 0.]);
        let mut node = Node::new_leaf(&origin, 5);
        get_test_points().iter().for_each(|point_coords| {
            node.points_mut()
                .push(Point::with_coords(point_coords.to_owned()));
        });

        let variance = calculate(&node, 0, node.immed_children());
        assert_eq!(variance[0], 0.);
        assert_eq!(variance[1], 14.);
    }

    #[test]
    pub fn test_range_variance_calculation() {
        let origin = Point::with_coords(vec![0., 0.]);
        let mut node = Node::new_leaf(&origin, 5);
        get_test_points().iter().for_each(|point_coords| {
            node.points_mut()
                .push(Point::with_coords(point_coords.to_owned()));
        });

        let variance = calculate(&node, 0, 2);
        assert_eq!(variance[0], 0.);
        assert_eq!(variance[1], 0.25);
    }
}
