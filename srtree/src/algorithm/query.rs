use crate::measure::distance::euclidean;
use crate::node::Node;
use ordered_float::{Float, OrderedFloat};
use priority_queue::{DoublePriorityQueue, PriorityQueue};
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

pub fn nearest_neighbors<T>(
    node: &Node<T>,
    point: &[T],
    k: usize,
    distance: T
) -> (Vec<Vec<T>>, T)
where
    T: Debug + Float + AddAssign + SubAssign + MulAssign + DivAssign,
{
    if node.is_leaf() {
        let mut result = Vec::new();
        let mut max_distance = T::min_value();
        node.points().iter().for_each(|candidate| {
            result.push(candidate.clone());
            max_distance = max_distance.max(euclidean(candidate, point));
        });
        (result, max_distance)
    } else {
        let mut result = Vec::new();
        let mut max_distance = T::min_value();
        let mut target_distance = distance;

        // construct a queue with distance as a priority
        let mut queue: DoublePriorityQueue<usize, OrderedFloat<T>> = DoublePriorityQueue::new();
        for (index, candidate) in node.nodes().iter().enumerate() {
            queue.push(index, OrderedFloat(min_distance(candidate, point)));
        }

        while !queue.is_empty() {
            // pop the closest candidate
            let (candidate_index, candidate_min_dist) = queue.pop_min().unwrap();
            if candidate_min_dist > OrderedFloat(target_distance) || candidate_min_dist > OrderedFloat(distance) {
                break;
            }

            let (candidate_result, candidate_max_dist) = nearest_neighbors(&node.nodes()[candidate_index], point, k, target_distance);
            result.extend(candidate_result.to_owned());
            max_distance = max_distance.max(candidate_max_dist);

            if result.len() >= k {
                target_distance = max_distance;
            }
        }
        (result, target_distance)
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

        for i in 0..k {
            let point = vec![i as f64, 0.];
            assert!(result.contains(&point));
        }
    }
}
