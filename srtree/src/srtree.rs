use crate::algorithm::bulk_loading_omt::bulk_load;
use crate::algorithm::insertion::{insert_data, insert_node};
use crate::algorithm::query::nearest_neighbors;
use crate::algorithm::split::split;
use crate::node::Node;
use crate::params::Params;
use crate::shape::point::Point;
use ordered_float::Float;
use std::fmt::Debug;
use std::ops::{AddAssign, DivAssign, MulAssign, SubAssign};
use std::usize;

pub enum InsertionResult {
    Success,
    Failure,
}

pub struct SRTree<T> {
    root: Option<Node<T>>,
    params: Params,
}

impl<T> SRTree<T>
where
    T: Debug + Float + AddAssign + SubAssign + MulAssign + DivAssign,
{
    #[must_use]
    pub fn with_params(params: Params) -> SRTree<T> {
        SRTree { root: None, params }
    }

    #[must_use]
    pub fn new() -> SRTree<T> {
        SRTree {
            root: None,
            params: Params::default_params(),
        }
    }

    #[must_use]
    pub fn bulk_load(pts: &[Vec<T>], mut params: Params) -> SRTree<T> {
        let points: Vec<Point<T>> = pts
            .iter()
            .enumerate()
            .map(|(i, p)| Point::new(p.clone(), i))
            .collect();
        params.dimension = points[0].dimension();
        let root = bulk_load(points, &params);
        SRTree {
            root: Some(root),
            params,
        }
    }

    pub fn insert(&mut self, point_coords: &[T], index: usize) -> InsertionResult {
        if self.params.dimension == 0 {
            self.params.dimension = point_coords.len();
        }
        if self.params.dimension != point_coords.len() {
            eprintln!("Problem inserting a point: different dimensions");
            return InsertionResult::Failure;
        }
        if self.root.is_none() {
            self.root = Some(Node::new_leaf(
                &Point::with_coords(point_coords.to_vec()),
                self.params.max_number_of_elements,
            ));
        }

        if let Some(root) = self.root.as_mut() {
            insert_data(
                root,
                &Point::new(point_coords.to_vec(), index),
                &self.params,
            );

            if root.immed_children() > self.params.max_number_of_elements {
                let sibling = split(root, &root.get_sphere().center.clone(), &self.params);
                let mut new_root = Node::new_node(
                    &root.get_sphere().center,
                    self.params.max_number_of_elements,
                    root.get_height() + 1,
                );
                if let Some(old_root) = self.root.take() {
                    insert_node(&mut new_root, old_root, &self.params);
                }
                insert_node(&mut new_root, sibling, &self.params);
                self.root = Some(new_root);
            }
        }
        InsertionResult::Success
    }

    pub fn query(&self, point: &[T], k: usize) -> Vec<usize> {
        if let Some(root) = self.root.as_ref() {
            nearest_neighbors(root, &Point::with_coords(point.to_vec()), k)
        } else {
            Vec::new()
        }
    }

    pub fn node_count(&self) -> usize {
        if let Some(root) = self.root.as_ref() {
            root.node_count()
        } else {
            0
        }
    }

    pub fn leaf_count(&self) -> usize {
        if let Some(root) = self.root.as_ref() {
            root.leaf_count()
        } else {
            0
        }
    }
}

impl<T> Default for SRTree<T>
where
    T: Debug + Float + AddAssign + SubAssign + MulAssign + DivAssign,
{
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_insertion_query() {
        let params = Params::new(3, 7, 3, true).unwrap();
        let mut tree: SRTree<f64> = SRTree::with_params(params);
        let search_point = vec![1.0, 0.0];
        assert!(!tree.query(&search_point, 1).contains(&0)); // not inserted yet
        tree.insert(&search_point, 0);
        assert!(tree.query(&search_point, 1).contains(&0)); // inserted
    }
}
