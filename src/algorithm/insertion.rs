use crate::node::Node;
use ordered_float::Float;
use std::ops::{AddAssign, DivAssign, MulAssign, SubAssign};

pub fn insert<T>(node: &mut Node<T>, point: &Vec<T>)
where
    T: Float + AddAssign + SubAssign + MulAssign + DivAssign,
{
    if node.is_leaf() {
        node.points_mut().push(point.clone());
    }else{
        todo!("insert to a node");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::algorithm::query::nearest_neighbors;

    #[test]
    pub fn test_insertion() {
        let point = vec![0., 0.];
        let mut leaf_node = Node::new_leaf(&point, 10);
        insert(&mut leaf_node, &point);
        let mut result = Vec::new();
        nearest_neighbors(&mut leaf_node, &point, 10, &mut result);
        assert!(result.contains(&point));
    }
}