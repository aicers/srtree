use crate::measure::distance::euclidean;
use crate::node::Node;
use ordered_float::Float;
use std::ops::{AddAssign, DivAssign, MulAssign, SubAssign};

fn choose_closest_node_index<T>(node: &Node<T>, point: &Vec<T>) -> usize
where
    T: Float + AddAssign + SubAssign + MulAssign + DivAssign,
{
    let mut closest_node_index = 0;
    let mut distance = T::infinity();
    for (i, child) in node.nodes().iter().enumerate() {
        let current_distance = euclidean(&child.get_sphere().center, point);
        if current_distance < distance {
            distance = current_distance;
            closest_node_index = i;
        }
    }
    closest_node_index
}

pub fn choose_subtree<'a, T>(
    node: &'a mut Node<T>,
    search_node: &Node<T>,
    target_height: usize,
) -> &'a mut Node<T>
where
    T: Float + AddAssign + SubAssign + MulAssign + DivAssign,
{
    if node.is_leaf() || node.get_height() == target_height {
        return node;
    } else {
        // choose a node with the closest centroid to point
        let closest_node_index = choose_closest_node_index(node, &search_node.get_sphere().center);
        // descend until a leaf is reached
        choose_subtree(
            &mut node.nodes_mut()[closest_node_index],
            search_node,
            target_height,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_closest_node_selection() {
        let origin = vec![0., 0.];
        let mut node = Node::new_node(&origin, 10, 1);

        for i in 0..10 {
            let point = vec![i as f64, 0.];
            let child = Node::new_leaf(&point, 10);
            node.nodes_mut().push(child);
        }

        let expected_index = 9;
        let selected_index = choose_closest_node_index(&node, &vec![100., 0.]);
        assert_eq!(selected_index, expected_index);
    }
}
