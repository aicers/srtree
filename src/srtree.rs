use crate::algorithm::query::nearest_neighbors;
use crate::params::Params;
use crate::{algorithm::insertion::insert_data, node::Node};
use ordered_float::Float;
use std::ops::{AddAssign, DivAssign, MulAssign, SubAssign};

#[allow(dead_code)]
pub struct SRTree<T> {
    root: Option<Node<T>>,
    params: Params,
}

#[allow(dead_code)]
impl<T> SRTree<T>
where
    T: Float + AddAssign + SubAssign + MulAssign + DivAssign,
{
    #[must_use]
    pub fn new(
        min_number_of_elements: usize,
        max_number_of_elements: usize,
        reinsert_count: usize,
    ) -> SRTree<T> {
        SRTree {
            root: None,
            params: Params::new(
                min_number_of_elements,
                max_number_of_elements,
                reinsert_count,
            ),
        }
    }

    pub fn insert(&mut self, point: &Vec<T>) {
        if self.root.is_none() {
            self.root = Some(Node::new_leaf(point, self.params.max_number_of_elements));
        }
        insert_data(self.root.as_mut().unwrap(), point, &self.params);
    }

    pub fn query(&self, point: &Vec<T>, k: usize) -> Vec<Vec<T>> {
        let mut neigbors = Vec::with_capacity(k);
        if self.root.is_some() {
            nearest_neighbors(self.root.as_ref().unwrap(), point, k, &mut neigbors);
        }
        neigbors
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_insertion_query() {
        let mut tree: SRTree<f64> = SRTree::new(3, 10, 3);
        let search_point = vec![1.0, 0.0];
        assert!(!tree.query(&search_point, 1).contains(&search_point)); // not inserted yet
        tree.insert(&vec![1.0, 0.0]);
        assert!(tree.query(&search_point, 1).contains(&search_point)); // inserted
    }
}
