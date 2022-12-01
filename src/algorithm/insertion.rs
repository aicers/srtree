use crate::node::{Data, Node};
use crate::params::Params;
use crate::shape::reshape::reshape;
use ordered_float::Float;
use std::fmt::Debug;
use std::ops::{AddAssign, DivAssign, MulAssign, SubAssign};

use super::choose_subtree::choose_closest_node_index;
use super::split::split;

enum Reinsert<T> {
    NotRequired,
    Nodes(Vec<Node<T>>),
}

enum Split<T> {
    NotRequired,
    NewSibling(Node<T>),
}

enum OverflowTreatment<T> {
    NotRequired,
    Reinsert(Vec<Node<T>>, usize),
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
    let mut reinsert_height = insert_node.get_height();
    let mut reinsert_list: Vec<Node<T>> = vec![insert_node];
    while let Some(node) = reinsert_list.pop() {
        let result = overflow_treatment(root, node, params, reinsert_height);
        match result {
            OverflowTreatment::NotRequired => {},
            OverflowTreatment::Reinsert(nodes_to_reinsert, new_reinsert_height) => {
                reinsert_list.extend(nodes_to_reinsert);
                reinsert_height = new_reinsert_height;
            },
            OverflowTreatment::Split(new_node) => {
                reinsert_list.push(new_node);
            }
        }
    }
}

fn overflow_treatment<'a, T>(
    root: &mut Node<T>,
    insert_node: Node<T>,
    params: &Params,
    reinsert_height: usize,
) -> OverflowTreatment<T>
where
    T: Debug + Float + AddAssign + SubAssign + MulAssign + DivAssign,
{
    if reinsert_height == insert_node.get_height() {
        // Increase reinsertion height to prevent future reinsertion attempts in the same level:
        let new_reinsert_height = reinsert_height + 1;
        let result = insert_or_reinsert(root, insert_node, params);
        match result {
            Reinsert::Nodes(nodes) => OverflowTreatment::Reinsert(nodes, new_reinsert_height),
            Reinsert::NotRequired => OverflowTreatment::NotRequired,
        }
    } else {
        let result = insert_or_split(root, insert_node, params, root.get_height());
        match result {
            Split::NewSibling(new_node) => OverflowTreatment::Split(new_node),
            Split::NotRequired => OverflowTreatment::NotRequired,
        }
    }
}

fn insert_or_reinsert<T>(
    parent: &mut Node<T>,
    insert_node: Node<T>,
    params: &Params,
) -> Reinsert<T>
where
    T: Debug + Float + AddAssign + SubAssign + MulAssign + DivAssign,
{
    let target_height = insert_node.get_height() + 1;
    if parent.get_height() < target_height {
        panic!("trying to insert_or_reinsert a node on lower subtree that its height");
    }
    if parent.get_height() == target_height {
        if parent.is_leaf() {
            parent.points_mut()
                .push(insert_node.get_sphere().center.clone());
        } else {
            parent.nodes_mut().push(insert_node);
        }
        reshape(parent);

        if parent.immed_children() <= params.max_number_of_elements {
            Reinsert::NotRequired
        } else {
            let mut nodes_to_reinsert = parent.pop_last(params.reinsert_count);
            if params.prefer_close_reinsert {
                nodes_to_reinsert.reverse();
            }
            Reinsert::Nodes(nodes_to_reinsert)
        }
    } else {
        let closest_child_index = choose_closest_node_index(parent, &insert_node);
        let closest_child = &mut parent.nodes_mut()[closest_child_index];
        let result = insert_or_reinsert(closest_child, insert_node, params);
        reshape(parent);
        result
    }
}

fn insert_or_split<T>(
    parent: &mut Node<T>,
    insert_node: Node<T>,
    params: &Params,
    tree_height: usize,
) -> Split<T>
where
    T: Debug + Float + AddAssign + SubAssign + MulAssign + DivAssign,
{
    let target_height = insert_node.get_height() + 1;
    if parent.get_height() < target_height {
        panic!("trying to insert_or_split a node on lower subtree that its height");
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

        // don't split if parent is the root or is not an overfilled node.
        if parent.immed_children() <= params.max_number_of_elements
            || parent.get_height() == tree_height
        {
            Split::NotRequired
        } else {
            let sibling = split(parent, &params);
            Split::NewSibling(sibling)
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
    use std::ops::Div;

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

        // The first leaf
        let first_leaf_points = vec![vec![1., 1.],vec![3., 1.],vec![1., 3.],vec![3., 3.]];
        let mut leaf_node1 = Node::new_leaf(&first_leaf_points[0], params.max_number_of_elements);
        for point in first_leaf_points {
            insert_data(&mut leaf_node1, &point, &params);
        }
        assert_eq!(leaf_node1.immed_children(), 4);
        assert_eq!(leaf_node1.get_rect().low, vec![1., 1.]);
        assert_eq!(leaf_node1.get_rect().high, vec![3., 3.]);
        assert_eq!(leaf_node1.get_sphere().center, vec![2., 2.]);
        assert_eq!(leaf_node1.get_sphere().radius, (2.0).sqrt());

        // The second leaf
        let second_leaf_points = vec![vec![5., 1.],vec![6., 2.]];
        let mut leaf_node2 = Node::new_leaf(&second_leaf_points[0], params.max_number_of_elements);
        for point in second_leaf_points {
            insert_data(&mut leaf_node2, &point, &params);
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
        assert_eq!(root.get_sphere().center, vec![3.1666666666666665, 1.8333333333333333]);
        assert_eq!(root.get_sphere().radius, 2.953340857778225);

        // These two points expands the second leaf
        let new_points = vec![vec![7., 3.], vec![8., 4.]];
        for point in new_points {
            insert_data(&mut root, &point, &params);
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
        let new_point = vec![9., 5.];
        insert_data(&mut root, &new_point, &params);

        assert_eq!(root.immed_children(), 3);
        assert_eq!(root.get_total_children(), 9);
        assert_eq!(root.get_rect().low, vec![1., 1.]);
        assert_eq!(root.get_rect().high, vec![9., 5.]);

        println!("--------After insertion--------");
        println!("Root centroid = {:?}, radius = {:?}", root.get_sphere().center, root.get_sphere().radius);
        for i in 0..root.immed_children() {
            println!("Node {:?} centroid = {:?}, radius = {:?}", i + 1, root.child_centroid(i), root.nodes()[i].get_sphere().radius);
            for j in 0..root.child_immed_children(i) {
                println!("{:?}", root.nodes()[i].points()[j]);
            }
        }
    }
}
