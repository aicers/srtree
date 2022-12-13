use crate::measure::distance::euclidean;
use crate::node::Node;
use ordered_float::{Float, OrderedFloat};
use priority_queue::DoublePriorityQueue;
use std::{
    fmt::Debug,
    ops::{AddAssign, DivAssign, MulAssign, SubAssign},
};

fn min_distance<T>(node: &Node<T>, point: &[T]) -> T
where
    T: Debug + Float + AddAssign + SubAssign + MulAssign + DivAssign,
{
    let ds = node.get_sphere().min_distance(point);
    let dr = node.get_rect().min_distance(point);
    ds.max(dr)
}

fn min_max_distance<T>(node: &Node<T>, point: &[T]) -> T
where
    T: Debug + Float + AddAssign + SubAssign + MulAssign + DivAssign,
{
    node.get_rect().min_max_distance(point)
}

pub fn nearest_neighbors<T>(node: &Node<T>, point: &[T], k: usize, distance: T) -> (Vec<Vec<T>>, T)
where
    T: Debug + Float + AddAssign + SubAssign + MulAssign + DivAssign,
{
    if !node.is_leaf() {
        // construct a queue with distance as a priority
        let mut queue: DoublePriorityQueue<usize, OrderedFloat<T>> = DoublePriorityQueue::new();
        for (index, candidate) in node.nodes().iter().enumerate() {
            queue.push(index, OrderedFloat(min_distance(node, point)));
        }

        let mut result = Vec::new();
        let mut target_distance = OrderedFloat(distance);
        let mut max_distance = T::infinity();
        while !queue.is_empty() {
            let (index, min_distance) = queue.pop_min().unwrap();

            // downward pruning: 
            // there are already k neighbors, no need to explore nodes with min_distance is bigger than result.max_distance
            if min_distance > target_distance {
                break
            }

            let (candidates, cand_max_distance) = nearest_neighbors(&node.nodes()[index], point, k, distance);
            result.extend(candidates);
            max_distance = max_distance.max(cand_max_distance);

            if result.len() >= k {
                target_distance = OrderedFloat(max_distance);
            }
        }

        result.sort_by_key(|neighbor| OrderedFloat(euclidean(neighbor, point)));
        (result, max_distance)
    } else {
        // construct a queue with distance as a priority
        let mut queue: DoublePriorityQueue<usize, OrderedFloat<T>> = DoublePriorityQueue::new();
        for (index, candidate) in node.points().iter().enumerate() {
            queue.push(index, OrderedFloat(euclidean(candidate, point)));
        }

        // keep selecting the next nearest item until k neighbors are found:
        let mut result = Vec::new();
        let mut max_distance = T::infinity();
        while !queue.is_empty() {
            let (index, distance) = queue.pop_min().unwrap();
            if result.len() < k {
                result.push(node.points()[index].clone());
                max_distance = max_distance.max(distance.0);
            } else {
                break;
            }
        }
        (result, max_distance)
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
        let (result, max_distance) = nearest_neighbors(&mut leaf_node, &origin, k, f64::infinity());

        assert!(result.len() == k);
        for i in 0..k {
            let point = vec![i as f64, 0.];
            assert!(result.contains(&point));
        }
    }
}
