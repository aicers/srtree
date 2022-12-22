use crate::measure::distance::euclidean_squared;
use crate::measure::mean;
use crate::node::Node;
use crate::shape::reshape::reshape;
use crate::{measure::variance::calculate, params::Params};
use ordered_float::{Float, OrderedFloat};
use std::{
    fmt::Debug,
    ops::{AddAssign, DivAssign, MulAssign, SubAssign},
};

fn choose_split_axis<T>(node: &Node<T>) -> usize
where
    T: Debug + Float + AddAssign + SubAssign + MulAssign + DivAssign,
{
    // 1. Calculate variance for each axis:
    let variance = calculate(node, 0, node.immed_children());

    // 2. Choose the axis with the highest variance
    let mut selected_index = 0;
    for i in 0..variance.len() {
        if variance[i] > variance[selected_index] {
            selected_index = i;
        }
    }
    selected_index
}

fn choose_split_index<T>(node: &Node<T>, params: &Params) -> usize
where
    T: Debug + Float + AddAssign + SubAssign + MulAssign + DivAssign,
{
    assert!(
        node.immed_children() >= 2 * params.min_number_of_elements,
        "trying to split a node with less elements"
    );

    // Minimize the sum of variances for two groups of node.points
    let mut selected_index = params.min_number_of_elements;
    let mut min_variance = T::infinity();

    let number_of_entries = node.immed_children();
    let start = params.min_number_of_elements;
    let end = params
        .max_number_of_elements
        .min(number_of_entries - params.min_number_of_elements)
        + 1;

    for i in start..end {
        let mut current_variance = T::zero();

        // first group
        for variance in calculate(node, 0, i) {
            current_variance += variance;
        }

        // second group
        for variance in calculate(node, i, number_of_entries) {
            current_variance += variance;
        }

        if current_variance < min_variance {
            min_variance = current_variance;
            selected_index = i;
        }
    }
    selected_index
}

pub fn split<T>(node: &mut Node<T>, parent_centroid: &[T], params: &Params) -> Node<T>
where
    T: Debug + Float + AddAssign + SubAssign + MulAssign + DivAssign,
{
    assert!(
        node.immed_children() >= 2 * params.min_number_of_elements,
        "don't split a node with less elements than min_num_of_elements"
    );

    // 1. Choose the split axis
    let axis = choose_split_axis(node);

    // 2. Sort node points along that axis
    if node.is_leaf() {
        node.points_mut().sort_by_key(|p| OrderedFloat(p[axis]));
    } else {
        node.nodes_mut()
            .sort_by_key(|child| OrderedFloat(child.get_sphere().center[axis]));
    }

    // 3. Choose the split index along this axis
    let mut index = choose_split_index(node, params);

    let node_centroid = mean::calculate(node, 0, index);
    let node_distance = euclidean_squared(parent_centroid, &node_centroid);

    let sibling_centroid = mean::calculate(node, index, node.immed_children());
    let sibling_distance = euclidean_squared(parent_centroid, &sibling_centroid);

    if node_distance > sibling_distance {
        if node.is_leaf() {
            node.points_mut().reverse();
        } else {
            node.nodes_mut().reverse();
        }
        index = node.immed_children() - index;
    }

    // 4. Pop entries from end until node has index elements
    let mut new_node = node.new_sibling(params.max_number_of_elements);
    while node.immed_children() > index {
        if new_node.is_leaf() {
            new_node.points_mut().push(node.points_mut().pop().unwrap());
        } else {
            new_node.nodes_mut().push(node.nodes_mut().pop().unwrap());
        }
    }
    reshape(node);
    reshape(&mut new_node);

    // todo: return the node whose centroid is closest to parent
    new_node
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
    pub fn test_node_mean_variance_calculation() {
        let max_number_of_elements = 5;
        let origin = vec![0., 0.];
        let mut node = Node::new_node(&origin, max_number_of_elements, 1);

        let mut leaf1 = Node::new_leaf(&origin, max_number_of_elements);
        leaf1.points_mut().push(vec![0., 0.]);
        leaf1.points_mut().push(vec![0., 1.]);
        reshape(&mut leaf1);

        node.nodes_mut().push(leaf1);

        let mut leaf2 = Node::new_leaf(&origin, max_number_of_elements);
        leaf2.points_mut().push(vec![0., 100.]);
        leaf2.points_mut().push(vec![0., 200.]);
        reshape(&mut leaf2);

        node.nodes_mut().push(leaf2);
        reshape(&mut node);

        assert_eq!(node.get_variance()[1], 6837.6875);
    }

    #[test]
    pub fn test_choose_split_axis() {
        let origin = vec![0., 0.];
        let mut node = Node::new_leaf(&origin, 5);
        get_test_points().iter().for_each(|point| {
            node.points_mut().push(point.to_owned());
        });

        let expected_axis = 1;
        let selected_axis = choose_split_axis(&node);
        assert_eq!(expected_axis, selected_axis);
    }

    #[test]
    pub fn test_choose_split_index() {
        let origin = vec![0., 0.];
        let mut node = Node::new_leaf(&origin, 5);
        get_test_points().iter().for_each(|point| {
            node.points_mut().push(point.to_owned());
        });

        let params = Params::new(1, 3, 1, true).unwrap();
        let expected_index = 3;
        let selected_index = choose_split_index(&node, &params);
        assert_eq!(expected_index, selected_index);
    }

    #[test]
    pub fn test_split_leaf_node() {
        let params = Params::new(2, 5, 2, true).unwrap();

        let origin = vec![0., 0.];
        let mut node = Node::new_leaf(&origin, params.max_number_of_elements);
        get_test_points().iter().for_each(|point| {
            node.points_mut().push(point.to_owned());
        });

        let sibling = split(&mut node, &origin, &params);
        assert_eq!(node.immed_children(), 3);
        assert_eq!(sibling.immed_children(), 2);
    }

    #[test]
    pub fn test_split_node() {
        let params = Params::new(2, 5, 2, true).unwrap();

        let origin = vec![0., 0.];
        let mut node = Node::new_node(&origin, params.max_number_of_elements, 1);
        get_test_points().iter().for_each(|point| {
            let mut leaf = Node::new_leaf(point, params.max_number_of_elements);
            leaf.points_mut().push(point.to_owned());
            reshape(&mut leaf);
            node.nodes_mut().push(leaf);
        });
        reshape(&mut node);

        let sibling = split(&mut node, &origin, &params);
        assert_eq!(node.immed_children(), 3);
        assert_eq!(sibling.immed_children(), 2);
    }

    #[test]
    pub fn test_split_node_with_parent() {
        let params = Params::new(2, 5, 2, true).unwrap();

        let origin = vec![0., 10.];
        let mut node = Node::new_leaf(&origin, params.max_number_of_elements);
        node.points_mut().push(vec![0., 9.]);
        node.points_mut().push(vec![0., 8.]);
        node.points_mut().push(vec![0., 7.]);
        node.points_mut().push(vec![0., 6.]);
        node.points_mut().push(vec![0., 1.]);
        node.points_mut().push(vec![0., 2.]);
        reshape(&mut node);

        let sibling = split(&mut node, &origin, &params);
        assert_eq!(node.immed_children(), 4);
        assert_eq!(sibling.immed_children(), 2);
    }
}
