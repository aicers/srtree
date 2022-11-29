use crate::node::{Data, Node};
use crate::params::Params;
use crate::shape::reshape::reshape;
use ordered_float::Float;
use std::fmt::Debug;
use std::ops::{AddAssign, DivAssign, MulAssign, SubAssign};

use super::choose_subtree::choose_closest_node_index;
use super::split::split;

pub enum InsertionResult<T> {
    Success,
    Reinsert(Vec<Node<T>>),
}

pub enum ReinsertResult<T> {
    Success,
    Split(Node<T>),
}

pub fn insert_data<T>(node: &mut Node<T>, point: &Vec<T>, params: &Params)
where
    T: Debug + Float + AddAssign + SubAssign + MulAssign + DivAssign,
{
    insert(node, Node::new_point(point), params);
}

pub fn insert<'a, T>(root: &mut Node<T>, insert_node: Node<T>, params: &Params)
where
    T: Debug + Float + AddAssign + SubAssign + MulAssign + DivAssign,
{
    let result = insert_or_reinsert(root, insert_node, params);
    let mut reinsert_list: Vec<Node<T>> = vec![];
    match result {
        InsertionResult::Reinsert(nodes_to_reinsert) => {
            reinsert_list.extend(nodes_to_reinsert);
        }
        InsertionResult::Success => {}
    }

    // todo: reinsert a splitted node on higher levels
    while let Some(node) = reinsert_list.pop() {
        // if reinsertion fails, split the node this time
        let result = insert_or_split(root, node, params);
        match result {
            ReinsertResult::Split(new_node) => {
                reinsert_list.push(new_node);
            }
            ReinsertResult::Success => {}
        }
    }
}

fn insert_or_reinsert<T>(
    root: &mut Node<T>,
    insert_node: Node<T>,
    params: &Params,
) -> InsertionResult<T>
where
    T: Debug + Float + AddAssign + SubAssign + MulAssign + DivAssign,
{
    let target_height = insert_node.get_height() + 1;
    if root.get_height() < target_height {
        panic!("trying to insert a node on lower subtree that its height");
    }
    if root.get_height() == target_height {
        if root.is_leaf() {
            root.points_mut()
                .push(insert_node.get_sphere().center.clone());
        } else {
            root.nodes_mut().push(insert_node);
        }
        reshape(root);

        if root.immed_children() <= params.max_number_of_elements {
            InsertionResult::Success
        } else {
            InsertionResult::Reinsert(root.pop_last(params.reinsert_count))
        }
    } else {
        let closest_child_index = choose_closest_node_index(root, &insert_node);
        let closest_child = &mut root.nodes_mut()[closest_child_index];
        let result = insert_or_reinsert(closest_child, insert_node, params);
        reshape(root);
        result
    }
}

fn insert_or_split<T>(
    root: &mut Node<T>,
    insert_node: Node<T>,
    params: &Params,
) -> ReinsertResult<T>
where
    T: Debug + Float + AddAssign + SubAssign + MulAssign + DivAssign,
{
    let target_height = insert_node.get_height() + 1;
    if root.get_height() < target_height {
        panic!("trying to insert a node on lower subtree that its height");
    }
    if root.get_height() == target_height {
        if root.is_leaf() {
            root.points_mut()
                .push(insert_node.get_sphere().center.clone());
        } else {
            root.nodes_mut().push(insert_node);
        }
        reshape(root);

        if root.immed_children() <= params.max_number_of_elements {
            ReinsertResult::Success
        } else {
            let sibling = split(root, &params);
            ReinsertResult::Split(sibling)
        }
    } else {
        let closest_child_index = choose_closest_node_index(&root, &insert_node);
        let closest_child = &mut root.nodes_mut()[closest_child_index];
        let result = insert_or_split(closest_child, insert_node, params);
        reshape(root);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::algorithm::{choose_subtree::choose_subtree, query::nearest_neighbors};

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
        let mut root = Node::new_node(&point, params.max_number_of_elements, 2);
        insert(&mut root, leaf_node1, &params);
        insert(&mut root, leaf_node2, &params);
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

    #[test]
    pub fn test_insert_overflow_treatment() {
        let params = Params::new(1, 4, 2);

        // first leaf
        let point = vec![0., 5.];
        let mut leaf_node1 = Node::new_leaf(&point, params.max_number_of_elements);
        insert_data(&mut leaf_node1, &point, &params);
        assert_eq!(leaf_node1.points().len(), 1);

        // second leaf
        let point = vec![0., 6.];
        let mut leaf_node2 = Node::new_leaf(&point, params.max_number_of_elements);
        insert_data(&mut leaf_node2, &point, &params);
        assert_eq!(leaf_node2.points().len(), 1);

        // insert the leaves
        let point = vec![0., 0.];
        let mut root = Node::new_node(&point, params.max_number_of_elements, 2);
        insert(&mut root, leaf_node1, &params);
        insert(&mut root, leaf_node2, &params);
        assert_eq!(root.nodes().len(), 2);

        for i in 1..5 {
            let new_point = vec![0., i as f64];
            insert_data(&mut root, &new_point, &params);
        }
 
        // the first leaf node will be overfilled and some of its children will be reinserted to the second leaf
        // Initial:             5  6
        // After insertion: 12345  6
        // After reinsert:  1234  56
        assert_eq!(root.immed_children(), 2);
        assert_eq!(root.nodes()[0].immed_children(), 4);
        assert_eq!(root.nodes()[1].immed_children(), 2);
    }
}
