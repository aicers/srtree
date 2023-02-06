use crate::node::Node;
use crate::params::Params;
use crate::shape::point::Point;
use crate::shape::reshape::reshape;
use ordered_float::Float;
use std::fmt::Debug;
use std::ops::{AddAssign, DivAssign, MulAssign, SubAssign};

use super::choose_subtree::choose_closest_node_index;
use super::split::split;

enum OverflowTreatment<T> {
    NotRequired,
    Reinsert(Vec<Node<T>>),
    Split(Node<T>),
}

pub fn insert_data<T>(node: &mut Node<T>, point: &Point<T>, params: &Params)
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

fn insert<T>(root: &mut Node<T>, insert_node: Node<T>, params: &Params)
where
    T: Debug + Float + AddAssign + SubAssign + MulAssign + DivAssign,
{
    let mut reinsert_height = insert_node.get_height();
    let mut reinsert_list: Vec<Node<T>> = vec![insert_node];
    while let Some(node) = reinsert_list.pop() {
        let result = overflow_treatment(root, node, params, reinsert_height);
        match result {
            OverflowTreatment::NotRequired => {}
            OverflowTreatment::Reinsert(nodes_to_reinsert) => {
                reinsert_list.extend(nodes_to_reinsert);
                // Increase reinsertion height to prevent future reinsertion attempts in the same level:
                reinsert_height += 1;
            }
            OverflowTreatment::Split(new_node) => {
                reinsert_list.push(new_node);
            }
        }
    }
}

fn overflow_treatment<T>(
    root: &mut Node<T>,
    insert_node: Node<T>,
    params: &Params,
    reinsert_height: usize,
) -> OverflowTreatment<T>
where
    T: Debug + Float + AddAssign + SubAssign + MulAssign + DivAssign,
{
    if reinsert_height == insert_node.get_height() {
        insert_or_reinsert(root, insert_node, params)
    } else {
        insert_or_split(
            &root.get_sphere().center.clone(),
            root,
            insert_node,
            params,
            root.get_height(),
        )
    }
}

fn insert_or_reinsert<T>(
    node: &mut Node<T>,
    insert_node: Node<T>,
    params: &Params,
) -> OverflowTreatment<T>
where
    T: Debug + Float + AddAssign + SubAssign + MulAssign + DivAssign,
{
    let target_height = insert_node.get_height() + 1;
    assert!(
        node.get_height() >= target_height,
        "trying to insert_or_reinsert a node on lower subtree that its height"
    );

    if node.get_height() == target_height {
        if node.is_leaf() {
            node.points_mut().push(Point::new(
                insert_node.get_sphere().center.clone(),
                insert_node.get_total_children(),
            ));
        } else {
            node.nodes_mut().push(insert_node);
        }
        reshape(node);

        if node.immed_children() <= params.max_number_of_elements {
            OverflowTreatment::NotRequired
        } else {
            let mut nodes_to_reinsert = node.pop_last(params.reinsert_count);
            if params.prefer_close_reinsert {
                nodes_to_reinsert.reverse();
            }
            OverflowTreatment::Reinsert(nodes_to_reinsert)
        }
    } else {
        let closest_child_index = choose_closest_node_index(node, &insert_node);
        let closest_child = &mut node.nodes_mut()[closest_child_index];
        let result = insert_or_reinsert(closest_child, insert_node, params);
        reshape(node);
        result
    }
}

fn insert_or_split<T>(
    parent_centroid: &[T],
    node: &mut Node<T>,
    insert_node: Node<T>,
    params: &Params,
    tree_height: usize,
) -> OverflowTreatment<T>
where
    T: Debug + Float + AddAssign + SubAssign + MulAssign + DivAssign,
{
    let target_height = insert_node.get_height() + 1;
    assert!(
        node.get_height() >= target_height,
        "trying to insert_or_split a node on lower subtree that its height"
    );

    if node.get_height() == target_height {
        if node.is_leaf() {
            node.points_mut().push(Point::new(
                insert_node.get_sphere().center.clone(),
                insert_node.get_total_children(),
            ));
        } else {
            node.nodes_mut().push(insert_node);
        }
        reshape(node);

        // don't split if parent is the root or is not an overfilled node.
        if node.immed_children() <= params.max_number_of_elements
            || node.get_height() == tree_height
        {
            OverflowTreatment::NotRequired
        } else {
            let sibling = split(node, parent_centroid, params);
            OverflowTreatment::Split(sibling)
        }
    } else {
        let closest_child_index = choose_closest_node_index(node, &insert_node);
        let parent_centroid = node.get_sphere().center.clone();
        let closest_child = &mut node.nodes_mut()[closest_child_index];
        let result = insert_or_split(
            &parent_centroid,
            closest_child,
            insert_node,
            params,
            tree_height,
        );
        reshape(node);
        result
    }
}

#[cfg(test)]
mod tests {
    use std::{ops::Div, sync::Arc};

    use super::*;
    use crate::algorithm::choose_subtree::choose_subtree;

    #[test]
    pub fn test_leaf_insertion() {
        let point = Point::with_coords(vec![0., 0.]);
        let params = Params::new(4, 9, 4, true).unwrap();
        let mut leaf_node = Node::new_leaf(&point.coords, params.max_number_of_elements);
        insert_data(&mut leaf_node, &point, &params);
        assert_eq!(leaf_node.points()[0].coords, point.coords);
    }

    #[test]
    pub fn test_insert_now() {
        let params = Params::new(1, 10, 4, true).unwrap();

        // first leaf
        let point = Point::with_coords(vec![0., 0.]);
        let mut leaf_node1 = Node::new_leaf(&point.coords, params.max_number_of_elements);
        insert_data(&mut leaf_node1, &point, &params);
        assert_eq!(leaf_node1.points().len(), 1);

        // second leaf
        let point = Point::with_coords(vec![0., 10.]);
        let mut leaf_node2 = Node::new_leaf(&point.coords, params.max_number_of_elements);
        insert_data(&mut leaf_node2, &point, &params);
        assert_eq!(leaf_node2.points().len(), 1);

        // insert the leaves
        let mut root = Node::new_node(&point.coords, params.max_number_of_elements, 2);
        insert(&mut root, leaf_node1, &params);
        insert(&mut root, leaf_node2, &params);
        assert_eq!(root.nodes().len(), 2);

        // insert another point that's close to the second leaf
        let point = Point::with_coords(vec![0., 11.]);
        insert_data(&mut root, &point, &params);

        // search the point
        let search_node = Node::new_point(&point);
        let leaf = choose_subtree(&mut root, &search_node);
        assert_eq!(leaf.points().len(), 2);
        assert_eq!(leaf.get_sphere().center, vec![0., 10.5]);
        assert_eq!(root.get_sphere().center, vec![0., 7.]);
        assert_eq!(root.get_rect().low, vec![0., 0.]);
        assert_eq!(root.get_rect().high, vec![0., 11.]);
    }

    #[test]
    pub fn test_insert_overflow_treatment() {
        let params = Params::new(1, 4, 2, true).unwrap();

        // first leaf
        let point = Point::with_coords(vec![0., 0.]);
        let mut leaf_node1 = Node::new_leaf(&point.coords, params.max_number_of_elements);
        insert_data(&mut leaf_node1, &point, &params);
        assert_eq!(leaf_node1.points().len(), 1);

        // second leaf
        let point = Point::with_coords(vec![0., 1.]);
        let mut leaf_node2 = Node::new_leaf(&point.coords, params.max_number_of_elements);
        insert_data(&mut leaf_node2, &point, &params);
        assert_eq!(leaf_node2.points().len(), 1);

        // insert the leaves
        let point = vec![0., 0.];
        let mut root = Node::new_node(&point, params.max_number_of_elements, 2);
        insert(&mut root, leaf_node1, &params);
        insert(&mut root, leaf_node2, &params);
        assert_eq!(root.nodes().len(), 2);

        for i in 2..6 {
            let new_point = Point::with_coords(vec![0., i as f64]);
            insert_data(&mut root, &new_point, &params);
        }

        // the second leaf node will be overfilled and one of its points will be reinserted to the first leaf
        // Initial:         0  1
        // After insertion: 0  12345  <- overflow!
        // After reinsert:  01  2345
        assert_eq!(root.immed_children(), 2);
        assert_eq!(root.nodes()[0].immed_children(), 2);
        assert_eq!(root.nodes()[1].immed_children(), 4);
    }

    #[test]
    pub fn test_insert_dynamic_reorganization() {
        let params = Params::new(1, 4, 2, true).unwrap();

        // The first leaf
        let first_leaf_points = vec![vec![1., 1.], vec![3., 1.], vec![1., 3.], vec![3., 3.]];
        let mut leaf_node1 = Node::new_leaf(&first_leaf_points[0], params.max_number_of_elements);
        for point_coords in first_leaf_points {
            insert_data(&mut leaf_node1, &Point::with_coords(point_coords), &params);
        }
        assert_eq!(leaf_node1.immed_children(), 4);
        assert_eq!(leaf_node1.get_rect().low, vec![1., 1.]);
        assert_eq!(leaf_node1.get_rect().high, vec![3., 3.]);
        assert_eq!(leaf_node1.get_sphere().center, vec![2., 2.]);
        assert_eq!(leaf_node1.get_sphere().radius, (2.0).sqrt());

        // The second leaf
        let second_leaf_points = vec![vec![5., 1.], vec![6., 2.]];
        let mut leaf_node2 = Node::new_leaf(&second_leaf_points[0], params.max_number_of_elements);
        for point_coords in second_leaf_points {
            insert_data(&mut leaf_node2, &Point::with_coords(point_coords), &params);
        }
        assert_eq!(leaf_node2.immed_children(), 2);
        assert_eq!(leaf_node2.get_rect().low, vec![5., 1.]);
        assert_eq!(leaf_node2.get_rect().high, vec![6., 2.]);
        assert_eq!(leaf_node2.get_sphere().center, vec![5.5, 1.5]);
        assert_eq!(leaf_node2.get_sphere().radius, (2.0).sqrt().div(2.));

        // Insert the leaves
        let point = vec![0., 0.];
        let mut root = Node::new_node(&point, params.max_number_of_elements, 2);
        insert(&mut root, leaf_node1, &params);
        insert(&mut root, leaf_node2, &params);
        assert_eq!(root.immed_children(), 2);
        assert_eq!(root.get_total_children(), 6);
        assert_eq!(root.get_rect().low, vec![1., 1.]);
        assert_eq!(root.get_rect().high, vec![6., 3.]);
        assert_eq!(
            root.get_sphere().center,
            vec![3.1666666666666665, 1.8333333333333333]
        );
        assert_eq!(root.get_sphere().radius, 2.953340857778225);

        // These two points expands the second leaf
        let new_points = vec![vec![7., 3.], vec![8., 4.]];
        for point_coords in new_points {
            insert_data(&mut root, &Point::with_coords(point_coords), &params);
        }

        assert_eq!(root.immed_children(), 2);
        assert_eq!(root.get_total_children(), 8);
        assert_eq!(root.get_rect().low, vec![1., 1.]);
        assert_eq!(root.get_rect().high, vec![8., 4.]);
        assert_eq!(root.nodes()[1].immed_children(), 4);
        assert_eq!(root.nodes()[1].get_rect().low, vec![5., 1.]);
        assert_eq!(root.nodes()[1].get_rect().high, vec![8., 4.]);
        assert_eq!(root.nodes()[1].get_sphere().center, vec![6.5, 2.5]);
        assert_eq!(root.nodes()[1].get_sphere().radius, (18.0).sqrt().div(2.));

        // This insertion causes reinsert and split:
        let new_point = Point::with_coords(vec![9., 5.]);
        insert_data(&mut root, &new_point, &params);

        assert_eq!(root.immed_children(), 3);
        assert_eq!(root.get_total_children(), 9);
        assert_eq!(root.get_rect().low, vec![1., 1.]);
        assert_eq!(root.get_rect().high, vec![9., 5.]);
    }
}
