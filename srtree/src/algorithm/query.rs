use crate::measure::distance::euclidean_squared;
use crate::node::Node;
use ordered_float::{Float, OrderedFloat};
use std::{
    cmp::Ordering,
    collections::BinaryHeap,
    fmt::Debug,
    ops::{AddAssign, DivAssign, MulAssign, SubAssign},
};

struct Neighbor<T>
where
    T: Float,
{
    pub distance: OrderedFloat<T>,
    pub point: Vec<T>,
}

impl<T> Neighbor<T>
where
    T: Float,
{
    pub fn new(distance: OrderedFloat<T>, point: Vec<T>) -> Neighbor<T> {
        Neighbor { distance, point }
    }
}

impl<T> Ord for Neighbor<T>
where
    T: Float,
{
    #[must_use]
    fn cmp(&self, other: &Self) -> Ordering {
        self.distance.cmp(&other.distance)
    }
}

impl<T> PartialOrd for Neighbor<T>
where
    T: Float,
{
    #[must_use]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.distance.partial_cmp(&other.distance)
    }
}

impl<T> Eq for Neighbor<T> where T: Float {}

impl<T> PartialEq for Neighbor<T>
where
    T: Float,
{
    #[must_use]
    fn eq(&self, other: &Self) -> bool {
        self.distance.eq(&other.distance)
    }
}

pub fn nearest_neighbors<T>(node: &Node<T>, point: &[T], k: usize) -> Vec<Vec<T>>
where
    T: Debug + Float + AddAssign + SubAssign + MulAssign + DivAssign,
{
    let mut result = Vec::new();
    let mut distance_heap = BinaryHeap::new();
    search(node, point, k, &mut distance_heap);
    while !distance_heap.is_empty() {
        let last = distance_heap.pop().unwrap();
        result.push(last.point);
    }
    result.reverse();
    result
}

fn search<T>(node: &Node<T>, point: &[T], k: usize, distance_heap: &mut BinaryHeap<Neighbor<T>>)
where
    T: Debug + Float + AddAssign + SubAssign + MulAssign + DivAssign,
{
    if node.is_leaf() {
        // insert all potential neighbors (with their distances) in a leaf node:
        node.points().iter().for_each(|candidate| {
            let neighbor_distance = euclidean_squared(candidate, point);
            distance_heap.push(Neighbor::new(
                OrderedFloat(neighbor_distance),
                candidate.clone(),
            ));
        });

        // keep only closest k distances:
        while distance_heap.len() > k {
            distance_heap.pop();
        }
    } else {
        let mut to_visit = Vec::new();
        for (child_index, child) in node.nodes().iter().enumerate() {
            let distance = child.min_distance(point);
            to_visit.push((OrderedFloat(distance), child_index));
        }
        to_visit.sort();

        for (child_distance, child_index) in to_visit {
            // if k neighbors were already sampled, then the target distance is kth closest distance:
            let mut target_distance = OrderedFloat(T::infinity());
            if distance_heap.len() == k {
                target_distance = distance_heap.peek().unwrap().distance;
            }

            // search pruning: don't visit nodes with min_distance bigger than kth distance
            if child_distance > target_distance {
                break;
            }

            search(&node.nodes()[child_index], point, k, distance_heap);
        }
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
