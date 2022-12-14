use crate::algorithm::insertion::{insert_data, insert_node};
use crate::algorithm::query::nearest_neighbors;
use crate::algorithm::split::split;
use crate::measure::distance::euclidean;
use crate::node::Node;
use crate::params::Params;
use ordered_float::{Float, OrderedFloat};
use std::fmt::Debug;
use std::ops::{AddAssign, DivAssign, MulAssign, SubAssign};

pub enum InsertionResult {
    Success,
    Failure,
}

pub struct SRTree<T> {
    dimension: usize,
    root: Option<Node<T>>,
    params: Params,
}

#[allow(dead_code)]
impl<T> SRTree<T>
where
    T: Debug + Float + AddAssign + SubAssign + MulAssign + DivAssign,
{
    #[must_use]
    pub fn new(dimension: usize, params: Params) -> SRTree<T> {
        SRTree {
            root: None,
            params,
            dimension,
        }
    }

    pub fn insert(&mut self, point: &[T]) -> InsertionResult {
        if self.dimension != point.len() {
            eprintln!("Problem inserting a point: different dimensions");
            return InsertionResult::Failure;
        }
        if self.root.is_none() {
            self.root = Some(Node::new_leaf(point, self.params.max_number_of_elements));
        }

        if let Some(root) = self.root.as_mut() {
            insert_data(root, point, &self.params);

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

    pub fn query_all(&mut self, point: &[T], k: usize) -> Vec<Vec<T>> {
        if let Some(root) = self.root.as_mut() {
            let (candidates, _) = nearest_neighbors(root, point, k, T::infinity());
            let mut result = candidates.to_owned();
            result.sort_by_key(|candidate| OrderedFloat(euclidean(&point, &candidate)));
            result
        } else {
            Vec::new()
        }
    }
    
    pub fn query(&mut self, point: &[T], k: usize) -> Vec<Vec<T>> {
        let mut result = self.query_all(point, k);
        result.truncate(k);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_insertion_query() {
        let params = Params::new(3, 7, 3, true).unwrap();
        let mut tree: SRTree<f64> = SRTree::new(2, params);
        let search_point = vec![1.0, 0.0];
        assert!(!tree.query(&search_point, 1).contains(&search_point)); // not inserted yet
        tree.insert(&vec![1.0, 0.0]);
        assert!(tree.query(&search_point, 1).contains(&search_point)); // inserted
    }
}
