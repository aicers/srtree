use crate::node::Node;
use ordered_float::{Float, OrderedFloat};
use std::{
    fmt::Debug,
    ops::{AddAssign, DivAssign, MulAssign, SubAssign},
};

fn calculate_mean<T>(node: &Node<T>, from: usize, end: usize) -> Vec<T>
where
    T: Debug + Float + AddAssign + SubAssign + MulAssign + DivAssign,
{
    let mut number_of_entries = T::zero();
    let mut mean = vec![T::zero(); node.child_centroid(0).len()];
    for child_index in from..end {
        let child_number_of_entries = T::from(node.child_immed_children(child_index)).unwrap_or(T::one());
        for axis_index in 0..mean.len() {
            mean[axis_index] += node.child_centroid(child_index)[axis_index] * child_number_of_entries;
        }
        number_of_entries += child_number_of_entries;
    }
    for axis_index in 0..mean.len() {
        mean[axis_index] /= number_of_entries;
    }
    mean
}

fn calculate_variance<T>(node: &Node<T>, from: usize, end: usize) -> Vec<T>
where
    T: Debug + Float + AddAssign + SubAssign + MulAssign + DivAssign,
{
    if node.immed_children() == 0 || node.immed_children() < end || from >= end{
        return Vec::new();
    }

    // 1. Calculate mean (mean of entries)
    let mean = calculate_mean(node, from, end);

    // 2. Calculate variance w.r.t. the mean
    let mut number_of_entries = T::zero();
    let mut variance = vec![T::zero(); mean.len()];
    for child_index in from..end {
        let child_number_of_entries = T::from(node.child_immed_children(child_index)).unwrap_or(T::one());
        for axis_index in 0..variance.len() {
            variance[axis_index] += (node.child_centroid(child_index)[axis_index] - mean[axis_index]).powi(2) * child_number_of_entries;
            variance[axis_index] += child_number_of_entries * node.child_variance(child_index)[axis_index];
        }
        number_of_entries += child_number_of_entries;
    }
    for axis_index in 0..variance.len() {
        variance[axis_index] /= number_of_entries;
    }
    variance
}

fn choose_split_axis<T>(node: &Node<T>) -> usize
where
    T: Debug + Float + AddAssign + SubAssign + MulAssign + DivAssign,
{
    // 1. Calculate variance for each axis:
    let variance = calculate_variance(node, 0, node.immed_children());

    // 2. Choose the axis with the highest variance
    let mut selected_index = 0;
    for i in 0..variance.len() {
        if variance[i] > variance[selected_index] {
            selected_index = i;
        }
    }
    selected_index
}

fn choose_split_index<T>(
    node: &Node<T>,
    min_number_of_elements: usize,
    max_number_of_elements: usize,
) -> usize
where
    T: Debug + Float + AddAssign + SubAssign + MulAssign + DivAssign,
{
    if node.immed_children() < 2 * min_number_of_elements {
        panic!("Trying to split a node with less elements");
    }
    
    // Minimize the sum of variances for two groups of node.points
    let mut selected_index = min_number_of_elements;
    let mut min_variance = T::infinity();

    let number_of_entries = node.immed_children();
    let start = min_number_of_elements;
    let end = max_number_of_elements.min(number_of_entries - min_number_of_elements) + 1;

    for i in start..end {
        let mut current_variance = T::zero();

        // first group
        calculate_variance(node, 0, i)
            .iter()
            .for_each(|variance| {
                current_variance += variance.to_owned();
            });

        // second group
        calculate_variance(node, i, number_of_entries)
            .iter()
            .for_each(|variance| {
                current_variance += variance.to_owned();
            });

        if current_variance < min_variance {
            min_variance = current_variance;
            selected_index = i;
        }
    }
    selected_index
}

pub fn split<T>(node: &mut Node<T>,
    min_number_of_elements: usize,
    max_number_of_elements: usize,
) -> Option<Node<T>>
where
    T: Debug + Float + AddAssign + SubAssign + MulAssign + DivAssign,
{
    if node.immed_children() < 2 * min_number_of_elements {
        return None;
    }

    // 1. Choose the split axis
    let axis = choose_split_axis(node);

    // 2. Sort node points along that axis
    if node.is_leaf() {
        node.points_mut().sort_by_key(|p| OrderedFloat(p[axis]));
    }else{
        node.nodes_mut().sort_by_key(|child| OrderedFloat(child.centroid()[axis]));
    }

    // 3. Choose the split index along this axis
    let index = choose_split_index(node, min_number_of_elements, max_number_of_elements);

    // 4. Pop entries from end until node has index elements
    let mut new_node = Node::new_sibling(node, max_number_of_elements);
    while node.immed_children() > index {
        if new_node.is_leaf() {
            new_node.points_mut().push(node.points_mut().pop().unwrap());
        }else {
            new_node.nodes_mut().push(node.nodes_mut().pop().unwrap());
        }
    }
    let variance = calculate_variance(node, 0, node.immed_children());
    node.adjust_shape(variance);
    let variance = calculate_variance(&new_node, 0, node.immed_children());
    new_node.adjust_shape(variance);
    Some(new_node)
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
    pub fn test_variance_calculation() {
        let origin = vec![0., 0.];
        let mut node = Node::new_leaf(&origin, 5);
        get_test_points().iter().for_each(|point| {
            node.points_mut().push(point.to_owned());
        });

        let variance = calculate_variance(&node, 0, node.immed_children());
        assert_eq!(variance[0], 0.);
        assert_eq!(variance[1], 14.);
    }

    #[test]
    pub fn test_range_variance_calculation() {
        let origin = vec![0., 0.];
        let mut node = Node::new_leaf(&origin, 5);
        get_test_points().iter().for_each(|point| {
            node.points_mut().push(point.to_owned());
        });

        let variance = calculate_variance(&node, 0, 2);
        assert_eq!(variance[0], 0.);
        assert_eq!(variance[1], 0.25);
    }

    #[test]
    pub fn test_node_mean_variance_calculation(){
        let min_number_of_elements = 2;
        let max_number_of_elements = 5;
        let origin = vec![0., 0.];

        let mut node = Node::new_node(&origin, max_number_of_elements,1);
        for i in 0..max_number_of_elements {
            let mut leaf = Node::new_leaf(&origin, max_number_of_elements);
            for j in 0..max_number_of_elements {
                let point = vec![0., 10. * i as f64 + j as f64];
                leaf.points_mut().push(point.to_owned());
            }
            let variance = calculate_variance(&leaf, 0, leaf.immed_children());
            leaf.adjust_shape(variance);
            node.nodes_mut().push(leaf);
        }
        let variance = calculate_variance(&node, 0, node.immed_children());
        node.adjust_shape(variance);

        let mean = calculate_mean(&node, 0, node.immed_children());
        assert_eq!(mean[0], 0.);
        assert_eq!(mean[1], 22.);

        assert_eq!(node.get_variance()[0], 0.);
        assert_eq!(node.get_variance()[1], 202.);
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

        let expected_index = 3;
        let selected_index = choose_split_index(&node, 2, 3);
        assert_eq!(expected_index, selected_index);
    }

    #[test]
    pub fn test_split_leaf_node(){
        let min_number_of_elements = 2;
        let max_number_of_elements = 5;

        let origin = vec![0., 0.];
        let mut node = Node::new_leaf(&origin, max_number_of_elements);
        get_test_points().iter().for_each(|point| {
            node.points_mut().push(point.to_owned());
        });

        let sibling = split(&mut node, min_number_of_elements, max_number_of_elements);
        assert!(sibling.is_some());
        assert_eq!(node.immed_children(), 3);
        assert_eq!(sibling.unwrap().immed_children(), 2);
    }

    #[test]
    pub fn test_split_node(){
        let min_number_of_elements = 2;
        let max_number_of_elements = 5;

        let origin = vec![0., 0.];
        let mut node = Node::new_node(&origin, max_number_of_elements,1);
        get_test_points().iter().for_each(|point| {
            let mut leaf = Node::new_leaf(point, max_number_of_elements);
            leaf.points_mut().push(point.to_owned());
            leaf.adjust_shape(vec![0.; point.len()]);
            node.nodes_mut().push(leaf);
        });
        let variance = calculate_variance(&node, 0, node.immed_children());
        node.adjust_shape(variance);

        let sibling = split(&mut node, min_number_of_elements, max_number_of_elements);
        assert!(sibling.is_some());
        assert_eq!(node.immed_children(), 3);
        assert_eq!(sibling.unwrap().immed_children(), 2);
    }
}
