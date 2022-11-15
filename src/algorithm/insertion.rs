use crate::node::Node;
use crate::algorithm::distance::euclidean;
use ordered_float::Float;
use std::ops::{AddAssign, DivAssign, MulAssign, SubAssign};

fn choose_closest<T>(nodes: &Vec<Node<T>>, point: &Vec<T>) -> usize
where
    T: Float + AddAssign + SubAssign + MulAssign + DivAssign,
{
    // choose a node with the closest centroid to point
    let mut selected_index = 0;
    let mut distance = T::infinity();
    for (i, node) in nodes.iter().enumerate() {
        let current_distance = euclidean(&node.sphere().center, point);
        if current_distance < distance {
            distance = current_distance;
            selected_index = i;
        }
    }
    selected_index
}

fn choose_leaf<'a, T>(node: &'a mut Node<T>, point: &'a Vec<T>) -> &'a mut Node<T>
where
    T: Float + AddAssign + SubAssign + MulAssign + DivAssign,
{
    if node.is_leaf() {
        return node;
    }else {
        let selected_index = choose_closest(node.nodes(), point);
        // descend until a leaf is reached
        choose_leaf(&mut node.nodes_mut()[selected_index], point)
    }
}

pub fn insert<T>(node: &mut Node<T>, point: &Vec<T>)
where
    T: Float + AddAssign + SubAssign + MulAssign + DivAssign,
{
    let leaf = choose_leaf(node, point);
    leaf.points_mut().push(point.clone());
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
        nearest_neighbors(&mut leaf_node, &point, 1, &mut result);
        assert!(result.contains(&point));
    }
}