use crate::algorithm::distance::euclidean;
use crate::node::Node;
use ordered_float::Float;
use std::ops::{AddAssign, DivAssign, MulAssign, SubAssign};

fn choose_split_dimension<T>(node: &Node<T>) -> usize {
    // choose the dimension with the highest variance
    0
}

fn choose_split_index<T>(node: &Node<T>, dimension: usize) -> usize {
    0
}

pub fn split<'a, T>(node: &'a mut Node<T>) -> Option<Node<T>> {
    None
}

#[cfg(test)]
mod tests {
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
    pub fn test_choose_split_dimension() {
        let origin = vec![0., 0.];
        let mut node = Node::new_leaf(&origin, 10);
        get_test_points().iter().for_each(|point| {
            node.points_mut().push(point.to_owned());
        });

        let expected_dimension = 1;
        let selected_dimension = choose_split_dimension(&node);
        assert_eq!(expected_dimension, selected_dimension);
    }

    #[test]
    pub fn test_choose_split_index() {
        let origin = vec![0., 0.];
        let mut node = Node::new_leaf(&origin, 10);
        get_test_points().iter().for_each(|point| {
            node.points_mut().push(point.to_owned());
        });

        let expected_index = 3;
        let selected_index = choose_split_index(&node, 1);
        assert_eq!(expected_index, selected_index);
    }
}
