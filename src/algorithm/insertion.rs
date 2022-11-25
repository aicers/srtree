use crate::params::Params;
use crate::shape::reshape::reshape;
use crate::{algorithm::choose_subtree::choose_subtree, params};
use crate::node::Node;
use ordered_float::Float;
use std::ops::{AddAssign, DivAssign, MulAssign, SubAssign};

pub fn insert<T>(node: &mut Node<T>, insert_node: Node<T>, target_height: usize, params: &Params)
where
    T: Float + AddAssign + SubAssign + MulAssign + DivAssign,
{
    let node = choose_subtree(node, &insert_node, target_height);
    if node.immed_children() < params.max_number_of_elements {
        if node.is_leaf() {
            node.points_mut().push(insert_node.get_sphere().center.clone());
        }else{
            node.nodes_mut().push(insert_node);
        }
        reshape(node);
    }
}

pub fn insert_data<T>(node: &mut Node<T>, point: &Vec<T>, params: &Params)
where
    T: Float + AddAssign + SubAssign + MulAssign + DivAssign,
{
    insert(node, Node::new_point(point), 0, params);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::algorithm::query::nearest_neighbors;

    #[test]
    pub fn test_leaf_insertion() {
        let point = vec![0., 0.];
        let mut leaf_node = Node::new_leaf(&point, 10);
        let params = Params::new(4, 9, 4);
        insert_data(&mut leaf_node, &point, &params);
        let mut result = Vec::new();
        nearest_neighbors(&mut leaf_node, &point, 1, &mut result);
        assert!(result.contains(&point));
    }

    #[test]
    pub fn test_insertion() {
        let point = vec![0., 0.];
    }
}
