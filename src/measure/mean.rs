use crate::node::Node;
use ordered_float::Float;
use std::ops::{AddAssign, DivAssign, MulAssign, SubAssign};

pub fn calculate_mean<T>(node: &Node<T>, from: usize, end: usize) -> Vec<T>
where
    T: Float + AddAssign + SubAssign + MulAssign + DivAssign,
{
    let mut number_of_entries = T::zero();
    let mut mean = vec![T::zero(); node.dimension()];
    for child_index in from..end {
        let child_number_of_entries =
            T::from(node.child_immed_children(child_index)).unwrap_or(T::one());
        for axis_index in 0..mean.len() {
            mean[axis_index] +=
                node.child_centroid(child_index)[axis_index] * child_number_of_entries;
        }
        number_of_entries += child_number_of_entries;
    }
    for axis_index in 0..mean.len() {
        mean[axis_index] /= number_of_entries;
    }
    mean
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_leaf_mean_calculation() {
        let mut leaf = Node::new_leaf(&vec![0., 0.], 5);
        leaf.points_mut().push(vec![1., 0.]);
        leaf.points_mut().push(vec![0., 1.]);
        let mean = calculate_mean(&leaf, 0, leaf.immed_children());
        assert_eq!(mean, vec![0.5, 0.5]);
    }

    #[test]
    pub fn test_node_mean_calculation() {
        let mut leaf1 = Node::new_leaf(&vec![0., 1.], 5);
        leaf1.points_mut().push(vec![0., 0.]);
        leaf1.points_mut().push(vec![0., 1.]);
        leaf1.points_mut().push(vec![0., 2.]);
        let mut leaf2 = Node::new_leaf(&vec![0., 4.], 5);
        leaf2.points_mut().push(vec![0., 3.]);
        leaf2.points_mut().push(vec![0., 5.]);

        let mut node = Node::new_node(&vec![0., 0.], 5, 1);
        node.nodes_mut().push(leaf1);
        node.nodes_mut().push(leaf2);

        let mean = calculate_mean(&node, 0, node.immed_children());
        assert_eq!(mean, vec![0., 2.2]);
    }
}
