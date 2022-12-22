use crate::measure::distance::euclidean_squared;
use crate::node::Node;
use ordered_float::{Float, OrderedFloat};
use std::{
    collections::BinaryHeap,
    fmt::Debug,
    ops::{AddAssign, DivAssign, MulAssign, SubAssign},
};

pub fn nearest_neighbors<T>(node: &Node<T>, point: &[T], k: usize) -> Vec<Vec<T>>
where
    T: Debug + Float + AddAssign + SubAssign + MulAssign + DivAssign,
{
    let mut result = Vec::new();
    let mut distance_heap = BinaryHeap::new();
    search(node, point, k, &mut result, &mut distance_heap);
    result.sort_by_key(|neighbor| OrderedFloat(euclidean_squared(point, neighbor)));
    result
}

fn search<T>(
    node: &Node<T>,
    point: &[T],
    k: usize,
    result: &mut Vec<Vec<T>>,
    distance_heap: &mut BinaryHeap<OrderedFloat<T>>,
) where
    T: Debug + Float + AddAssign + SubAssign + MulAssign + DivAssign,
{
    if node.is_leaf() {
        // insert all potential neighbors in a leaf node:
        node.points().iter().for_each(|neighbor| {
            let neighbor_distance = euclidean_squared(neighbor, point);
            distance_heap.push(OrderedFloat(neighbor_distance));
            result.push(neighbor.clone());
        });

        // keep only closest k distances:
        while distance_heap.len() > k {
            distance_heap.pop();
        }
    } else {
        node.nodes().iter().for_each(|child| {
            // if k neighbors were already sampled, then the target distance is kth closest distance:
            let mut target_distance = OrderedFloat(T::infinity());
            if distance_heap.len() == k {
                target_distance = *distance_heap.peek().unwrap();
            }

            // search pruning: don't visit nodes with min_distance bigger than kth distance
            let distance_to_child = OrderedFloat(child.min_distance(point));
            if distance_to_child < target_distance {
                search(child, point, k, result, distance_heap);
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::algorithm::insertion::insert_data;
    use crate::node::Node;
    use crate::params::Params;

    #[test]
    pub fn test_nearest_neighbors_with_leaf() {
        let params = Params::new(4, 9, 4, true).unwrap();
        let origin = vec![0., 0.];
        let mut leaf_node = Node::new_leaf(&origin, params.max_number_of_elements);

        for i in 0..params.max_number_of_elements {
            let point = vec![i as f64, 0.];
            insert_data(&mut leaf_node, &point, &params);
        }

        let k = params.max_number_of_elements / 3;
        let result = nearest_neighbors(&mut leaf_node, &origin, k);

        for i in 0..k {
            let point = vec![i as f64, 0.];
            assert!(result.contains(&point));
        }
    }
}
