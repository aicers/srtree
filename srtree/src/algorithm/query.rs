use crate::node::Node;
use crate::shape::point::Point;
use crate::stats::{
    inc_compared_nodes, inc_compared_points, inc_visited_nodes, inc_visited_points,
};
use ordered_float::{Float, OrderedFloat};
use std::cmp::Reverse;
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
    pub point_index: usize,
}

impl<T> Neighbor<T>
where
    T: Float,
{
    pub fn new(distance: OrderedFloat<T>, point_index: usize) -> Neighbor<T> {
        Neighbor {
            distance,
            point_index,
        }
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

pub fn nearest_neighbors<T>(node: &Node<T>, point: &Point<T>, k: usize) -> Vec<usize>
where
    T: Debug + Float + AddAssign + SubAssign + MulAssign + DivAssign,
{
    let mut result = Vec::new();
    let mut neighbors = BinaryHeap::new();
    search(node, point, k, &mut neighbors);

    while !neighbors.is_empty() {
        let last = neighbors.pop().unwrap();
        result.push(last.point_index);
    }
    result.reverse();
    result
}

fn search<T>(node: &Node<T>, point: &Point<T>, k: usize, neighbors: &mut BinaryHeap<Neighbor<T>>)
where
    T: Debug + Float + AddAssign + SubAssign + MulAssign + DivAssign,
{
    inc_visited_nodes(node.is_leaf());

    let mut kth_distance = OrderedFloat(T::infinity());
    if node.is_leaf() {
        inc_compared_points(node.points().len());

        let distance_to_center = point.distance(&node.sphere.center);
        for candidate in node.points() {
            if neighbors.len() == k {
                kth_distance = neighbors.peek().unwrap().distance;
            }

            let ball_bound = (distance_to_center - candidate.radius).max(T::zero());
            let ball_bound = OrderedFloat(ball_bound);
            if ball_bound > kth_distance {
                break;
            }

            let neighbor_distance = OrderedFloat(point.distance(candidate));
            if neighbors.len() < k {
                neighbors.push(Neighbor::new(neighbor_distance, candidate.index));
            } else if neighbor_distance < kth_distance {
                neighbors.pop();
                neighbors.push(Neighbor::new(neighbor_distance, candidate.index));
            }

            inc_visited_points();
        }
    } else {
        let mut to_visit = Vec::new();
        for (child_index, child) in node.nodes().iter().enumerate() {
            let distance = OrderedFloat(child.min_distance(point));
            to_visit.push((distance, child_index));
        }
        to_visit.sort();

        for (child_distance, child_index) in to_visit {
            inc_compared_nodes(node.nodes()[child_index].is_leaf());

            // if k neighbors were already sampled, then the target distance is kth closest distance:
            if neighbors.len() == k {
                kth_distance = neighbors.peek().unwrap().distance;
            }

            // search pruning: don't visit nodes with min_distance bigger than kth distance
            if child_distance > kth_distance {
                break;
            }

            search(&node.nodes()[child_index], point, k, neighbors);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::algorithm::insertion::insert_data;
    use crate::node::Node;
    use crate::params::Params;
    use crate::shape::point::Point;

    #[test]
    pub fn test_binary_search() {
        let pts = vec![6, 5, 3, 2, 1, 0];

        let result = pts.binary_search_by_key(&Reverse(7), |b| Reverse(*b));
        assert_eq!(result, Err(0));
    }

    #[test]
    pub fn test_nearest_neighbors_with_leaf() {
        let params = Params::new(4, 9, 4, true).unwrap();
        let origin = Point::with_coords(vec![0., 0.]);
        let mut leaf_node = Node::new_leaf(&origin, params.max_number_of_elements);

        for i in 0..params.max_number_of_elements {
            let point = Point::new(vec![i as f64, 0.], i);
            insert_data(&mut leaf_node, &point, &params);
        }

        let k = params.max_number_of_elements / 3;
        let result = nearest_neighbors(&mut leaf_node, &origin, k);

        for i in 0..k {
            let point = Point::new(vec![i as f64, 0.], i);
            assert!(result.contains(&point.index));
        }
    }
}
