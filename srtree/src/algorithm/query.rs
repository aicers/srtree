use crate::node::Node;
use crate::shape::point::Point;
use crate::stats::{
    inc_compared_nodes, inc_compared_points, inc_visited_nodes, inc_visited_points,
};
use ordered_float::{Float, OrderedFloat};
use std::{cmp::Ordering, collections::BinaryHeap};

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

pub fn search_neighbors<T>(node: &Node<T>, point: &Point<T>, k: usize) -> (Vec<usize>, Vec<T>)
where
    T: Float + Send + Sync,
{
    let mut neighbors = BinaryHeap::with_capacity(k);
    search(node, point, k, &mut neighbors);

    let neighbors = neighbors.into_sorted_vec();
    let indices = neighbors.iter().map(|n| n.point_index).collect();
    let distances = neighbors.iter().map(|n| n.distance.into_inner()).collect();
    (indices, distances)
}

fn search<T>(node: &Node<T>, point: &Point<T>, k: usize, neighbors: &mut BinaryHeap<Neighbor<T>>)
where
    T: Float + Send + Sync,
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
    use crate::node::Node;
    use crate::shape::point::Point;

    #[test]
    pub fn test_query_neighbors() {
        let mut leaf1 = Vec::new();
        for i in 0..10 {
            leaf1.push(Point::new(vec![i as f64, i as f64], i));
        }
        let leaf1 = Node::create_leaf(leaf1);
        let mut leaf2 = Vec::new();
        for i in 10..20 {
            leaf2.push(Point::new(vec![i as f64, i as f64], i));
        }
        let leaf2 = Node::create_leaf(leaf2);

        let root = Node::create_parent(vec![leaf1, leaf2]);
        let neighbors = search_neighbors(&root, &Point::with_coords(vec![0.0, 0.0]), 5);
        assert_eq!(neighbors.0, vec![0, 1, 2, 3, 4]);
    }
}
