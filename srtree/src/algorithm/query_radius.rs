use crate::node::Node;
use crate::shape::point::Point;
use crate::stats::{
    inc_compared_nodes, inc_compared_points, inc_visited_nodes, inc_visited_points,
};
use ordered_float::{Float, OrderedFloat};
use std::fmt::Debug;

pub fn search_neighborhood<T>(node: &Node<T>, point: &Point<T>, radius: T) -> Vec<usize>
where
    T: Debug + Copy + Float + Send + Sync,
{
    let mut neighbors = Vec::new();
    search_radius(node, point, OrderedFloat(radius), &mut neighbors);
    neighbors
}

fn search_radius<T>(
    node: &Node<T>,
    point: &Point<T>,
    radius: OrderedFloat<T>,
    neighbors: &mut Vec<usize>,
) where
    T: Debug + Copy + Float + Send + Sync,
{
    inc_visited_nodes(node.is_leaf());

    if node.is_leaf() {
        inc_compared_points(node.points().len());

        let distance_to_center = point.distance(&node.sphere.center);
        for candidate in node.points() {
            let ball_bound = (distance_to_center - candidate.radius).max(T::zero());
            let ball_bound = OrderedFloat(ball_bound);
            if ball_bound > radius {
                break;
            }

            let neighbor_distance = OrderedFloat(point.distance(candidate));
            if neighbor_distance <= radius {
                neighbors.push(candidate.index);
            }

            inc_visited_points();
        }
    } else {
        node.nodes().iter().for_each(|child| {
            inc_compared_nodes(child.is_leaf());
            let distance = OrderedFloat(child.min_distance(point));
            if distance <= radius {
                search_radius(child, point, radius, neighbors);
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::node::Node;
    use crate::shape::point::Point;

    #[test]
    pub fn test_query_neighborhood() {
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
        let neighbors = search_neighborhood(&root, &Point::with_coords(vec![0.0, 0.0]), 5.0);
        assert_eq!(neighbors, vec![0, 1, 2, 3]);
    }
}
