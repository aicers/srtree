use crate::node::Node;
use crate::params::Params;
use crate::shape::reshape::reshape;
use crate::{algorithm::choose_subtree::choose_subtree, params};
use ordered_float::Float;
use std::ops::{AddAssign, DivAssign, MulAssign, SubAssign};

use super::choose_subtree::choose_closest_node_index;

pub fn insert_data<T>(node: &mut Node<T>, point: &Vec<T>, params: &Params)
where
    T: Float + AddAssign + SubAssign + MulAssign + DivAssign,
{
    insert(node, Node::new_point(point), 0, params);
}

pub fn insert<T>(node: &mut Node<T>, insert_node: Node<T>, target_height: usize, params: &Params)
where
    T: Float + AddAssign + SubAssign + MulAssign + DivAssign,
{
    let target_node = choose_subtree(node, &insert_node, target_height);
    if target_node.immed_children() < params.max_number_of_elements {
        insert_now(node, insert_node, target_height);
    } else {
        overflow_treatment(node, target_height, params);
    }
}

pub fn overflow_treatment<T>(node: &mut Node<T>, target_height: usize, params: &Params)
where
    T: Float + AddAssign + SubAssign + MulAssign + DivAssign,
{
    // reinsert or split
}

pub fn reinsert<T>(node: &mut Node<T>, target_height: usize, params: &Params)
where
    T: Float + AddAssign + SubAssign + MulAssign + DivAssign,
{
    // remove p elements, reinsert them again
}

fn insert_now<T>(node: &mut Node<T>, insert_node: Node<T>, target_height: usize)
where
    T: Float + AddAssign + SubAssign + MulAssign + DivAssign,
{
    if node.get_height() == target_height {
        if node.is_leaf() {
            node.points_mut()
                .push(insert_node.get_sphere().center.clone());
        } else {
            node.nodes_mut().push(insert_node);
        }
        reshape(node);
    } else {
        let closest_child_index = choose_closest_node_index(node, &insert_node);
        insert_now(
            &mut node.nodes_mut()[closest_child_index],
            insert_node,
            target_height,
        );
        reshape(node);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::algorithm::query::nearest_neighbors;

    #[test]
    pub fn test_leaf_insertion() {
        let point = vec![0., 0.];
        let params = Params::new(4, 9, 4);
        let mut leaf_node = Node::new_leaf(&point, params.max_number_of_elements);
        insert_data(&mut leaf_node, &point, &params);
        let mut result = Vec::new();
        nearest_neighbors(&mut leaf_node, &point, 1, &mut result);
        assert!(result.contains(&point));
    }

    #[test]
    pub fn test_insert_now() {
        let params = Params::new(1, 10, 4);

        // first leaf
        let point = vec![0., 0.];
        let mut leaf_node1 = Node::new_leaf(&point, params.max_number_of_elements);
        insert_data(&mut leaf_node1, &point, &params);
        assert_eq!(leaf_node1.points().len(), 1);

        // second leaf
        let point = vec![0., 10.];
        let mut leaf_node2 = Node::new_leaf(&point, params.max_number_of_elements);
        insert_data(&mut leaf_node2, &point, &params);
        assert_eq!(leaf_node2.points().len(), 1);

        // insert the leaves
        let mut root = Node::new_node(&point, params.max_number_of_elements, 1);
        insert(&mut root, leaf_node1, 1, &params);
        insert(&mut root, leaf_node2, 1, &params);
        assert_eq!(root.nodes().len(), 2);

        // insert another point that's close to the second leaf
        let point = vec![0., 11.];
        insert_data(&mut root, &point, &params);

        // search the point
        let search_node = Node::new_point(&point);
        let leaf = choose_subtree(&mut root, &search_node, 0);
        assert_eq!(leaf.points().len(), 2);
        assert_eq!(leaf.get_sphere().center, vec![0., 10.5]);
        assert_eq!(root.get_sphere().center, vec![0., 7.]);
        assert_eq!(root.get_rect().low, vec![0., 0.]);
        assert_eq!(root.get_rect().high, vec![0., 11.]);
    }
}
