use crate::measure::distance::euclidean_squared;
use crate::node::Node;
use ordered_float::Float;
use std::{
    fmt::Debug,
    ops::{AddAssign, DivAssign, MulAssign, SubAssign},
};

pub fn choose_closest_node_index<T>(node: &Node<T>, search_node: &Node<T>) -> usize
where
    T: Debug + Float + AddAssign + SubAssign + MulAssign + DivAssign,
{
    assert!(!node.is_leaf(), "Trying to choose from a leaf node");

    let mut closest_node_index = 0;
    let mut distance = T::infinity();
    for (i, child) in node.nodes().iter().enumerate() {
        let current_distance =
            euclidean_squared(&child.get_sphere().center, &search_node.get_sphere().center);
        if current_distance < distance {
            distance = current_distance;
            closest_node_index = i;
        }
    }
    closest_node_index
}

#[allow(dead_code)]
pub fn choose_subtree<'a, T>(node: &'a Node<T>, search_node: &Node<T>) -> &'a Node<T>
where
    T: Debug + Float + AddAssign + SubAssign + MulAssign + DivAssign,
{
    if node.is_leaf() || node.get_height() == search_node.get_height() + 1 {
        return node;
    }
    // choose a node with the closest centroid to point
    let closest_node_index = choose_closest_node_index(node, search_node);
    // descend until a leaf is reached
    choose_subtree(&node.nodes()[closest_node_index], search_node)
}

#[cfg(test)]
mod tests {
    use crate::shape::point::Point;

    use super::*;

    #[test]
    pub fn test_closest_node_selection() {
        let origin = Point::with_coords(vec![0., 0.]);
        let mut node = Node::new_node(&origin, 10, 1);

        for i in 0..10 {
            let point = Point::with_coords(vec![i as f64, 0.]);
            let child = Node::new_leaf(&point, 10);
            node.nodes_mut().push(child);
        }

        let expected_index = 9;
        let search_node = Node::new_point(&Point::with_coords(vec![100., 0.]));
        let selected_index = choose_closest_node_index(&node, &search_node);
        assert_eq!(selected_index, expected_index);
    }
}
