use crate::node::{Data, Node};
use crate::params::Params;
use crate::shape::reshape::reshape;
use ordered_float::Float;
use std::fmt::Debug;
use std::ops::{AddAssign, DivAssign, MulAssign, SubAssign};

use super::choose_subtree::choose_closest_node_index;
use super::split::split;

pub enum InsertOrReinsert<T> {
    Success,
    Reinsert(Vec<Node<T>>),
}

pub enum InsertOrSplit<T> {
    Success,
    Split(Node<T>),
}

pub fn insert_data<T>(node: &mut Node<T>, point: &Vec<T>, params: &Params)
where
    T: Debug + Float + AddAssign + SubAssign + MulAssign + DivAssign,
{
    insert(node, Node::new_point(point), params);
}

pub fn insert_node<T>(root: &mut Node<T>, node: Node<T>, params: &Params)
where
    T: Debug + Float + AddAssign + SubAssign + MulAssign + DivAssign,
{
    insert(root, node, params);
}

fn insert<'a, T>(root: &mut Node<T>, insert_node: Node<T>, params: &Params)
where
    T: Debug + Float + AddAssign + SubAssign + MulAssign + DivAssign,
{
    let mut target_height = insert_node.get_height();
    let mut reinsert_list: Vec<Node<T>> = vec![insert_node];
    while let Some(node) = reinsert_list.pop() {
        if node.get_height() == target_height {
            // If this is the first call of reinsert in the target height,
            // try to insert or request for reinsert if overflow happens:
            let result = insert_or_reinsert(root, node, params);
            match result {
                InsertOrReinsert::Reinsert(nodes_to_reinsert) => {
                    reinsert_list.extend(nodes_to_reinsert);
                    target_height += 1; // increase the height to prevent reinserts in the same height
                }
                InsertOrReinsert::Success => {}
            }
        } else {
            // If this is NOT the first call of reinsert in the target height,
            // try to insert or request for split if overflow happens:
            let result = insert_or_split(root, node, params, root.get_height());
            match result {
                InsertOrSplit::Split(new_node) => {
                    reinsert_list.push(new_node);
                }
                InsertOrSplit::Success => {}
            }
        }
    }
}

fn insert_or_reinsert<T>(
    root: &mut Node<T>,
    insert_node: Node<T>,
    params: &Params,
) -> InsertOrReinsert<T>
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
            InsertOrReinsert::Success
        } else {
            let mut nodes_to_reinsert = root.pop_last(params.reinsert_count);
            if params.prefer_close_reinsert {
                nodes_to_reinsert.reverse();
            }
            InsertOrReinsert::Reinsert(nodes_to_reinsert)
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
    parent: &mut Node<T>,
    insert_node: Node<T>,
    params: &Params,
    tree_height: usize,
) -> InsertOrSplit<T>
where
    T: Debug + Float + AddAssign + SubAssign + MulAssign + DivAssign,
{
    let target_height = insert_node.get_height() + 1;
    if parent.get_height() < target_height {
        panic!("trying to insert a node on lower subtree that its height");
    }
    if parent.get_height() == target_height {
        if parent.is_leaf() {
            parent
                .points_mut()
                .push(insert_node.get_sphere().center.clone());
        } else {
            parent.nodes_mut().push(insert_node);
        }
        reshape(parent);

        if parent.immed_children() <= params.max_number_of_elements
            || parent.get_height() == tree_height
        // don't split the root
        {
            InsertOrSplit::Success
        } else {
            let sibling = split(parent, &params);
            InsertOrSplit::Split(sibling)
        }
    } else {
        let closest_child_index = choose_closest_node_index(&parent, &insert_node);
        let closest_child = &mut parent.nodes_mut()[closest_child_index];
        let result = insert_or_split(closest_child, insert_node, params, tree_height);
        reshape(parent);
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
        let params = Params::new(4, 9, 4, true);
        let mut leaf_node = Node::new_leaf(&point, params.max_number_of_elements);
        insert_data(&mut leaf_node, &point, &params);
        let mut result = Vec::new();
        nearest_neighbors(&mut leaf_node, &point, 1, &mut result);
        assert!(result.contains(&point));
    }

    #[test]
    pub fn test_insert_now() {
        let params = Params::new(1, 10, 4, true);

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
        let params = Params::new(1, 4, 2, true);

        // first leaf
        let point = vec![0., 0.];
        let mut leaf_node1 = Node::new_leaf(&point, params.max_number_of_elements);
        insert_data(&mut leaf_node1, &point, &params);
        assert_eq!(leaf_node1.points().len(), 1);

        // second leaf
        let point = vec![0., 1.];
        let mut leaf_node2 = Node::new_leaf(&point, params.max_number_of_elements);
        insert_data(&mut leaf_node2, &point, &params);
        assert_eq!(leaf_node2.points().len(), 1);

        // insert the leaves
        let point = vec![0., 0.];
        let mut root = Node::new_node(&point, params.max_number_of_elements, 2);
        insert(&mut root, leaf_node1, &params);
        insert(&mut root, leaf_node2, &params);
        assert_eq!(root.nodes().len(), 2);

        for i in 2..6 {
            let new_point = vec![0., i as f64];
            insert_data(&mut root, &new_point, &params);
        }

        // the first leaf node will be overfilled and one of its children will be reinserted to the second leaf
        // Initial:         0  1
        // After insertion: 0  12345  <- overflow!
        // After reinsert:  01  2345
        assert_eq!(root.immed_children(), 2);
        assert_eq!(root.nodes()[0].immed_children(), 2);
        assert_eq!(root.nodes()[1].immed_children(), 4);
    }

    #[test]
    pub fn test_insert_dynamic_reorganization() {
        let params = Params::new(1, 4, 2, true);

        // first leaf
        let point = vec![0., 0.];
        let mut leaf_node1 = Node::new_leaf(&point, params.max_number_of_elements);
        insert_data(&mut leaf_node1, &point, &params);
        assert_eq!(leaf_node1.points().len(), 1);

        // second leaf
        let point = vec![0., 1.];
        let mut leaf_node2 = Node::new_leaf(&point, params.max_number_of_elements);
        insert_data(&mut leaf_node2, &point, &params);
        assert_eq!(leaf_node2.points().len(), 1);

        // insert the leaves
        let point = vec![0., 0.];
        let mut root = Node::new_node(&point, params.max_number_of_elements, 2);
        insert(&mut root, leaf_node1, &params);
        insert(&mut root, leaf_node2, &params);
        assert_eq!(root.nodes().len(), 2);

        for i in 2..20 {
            let new_point = vec![0., i as f64];
            insert_data(&mut root, &new_point, &params);
        }

        // the first leaf node will be overfilled and some of its children will be reinserted to the second leaf
        // Root created with 2 leaves:  0  1
        // After inserting 18 points: 0_1_2_3   4_5_6_7    8_9_10_11   12_13_14_15  16_17_18_19      => 5 leaves
        assert_eq!(root.immed_children(), 5);
    }
}
