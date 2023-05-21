use crate::node::Node;
use crate::shape::point::Point;
use ordered_float::Float;
use std::fmt::Debug;

pub fn search_neighborhood<T>(node: &Node<T>, point: &Point<T>, radius: T) -> Vec<usize>
where
    T: Debug + Copy + Float + Send + Sync,
{
    let mut neighbors = Vec::new();
    search_radius(node, point, radius, &mut neighbors);
    neighbors
}

fn search_radius<T>(node: &Node<T>, point: &Point<T>, radius: T, neighbors: &mut Vec<usize>)
where
    T: Debug + Copy + Float + Send + Sync,
{
    if node.is_leaf() {
        let distance_to_center = point.distance(&node.sphere.center);
        for candidate in node.points() {
            let ball_bound = (distance_to_center - candidate.radius).max(T::zero());
            if ball_bound > radius {
                break;
            }

            let neighbor_distance = point.distance(candidate);
            if neighbor_distance <= radius {
                neighbors.push(candidate.index);
            }
        }
    } else {
        node.nodes()
            .iter()
            .filter(|child| child.min_distance(point) <= radius)
            .for_each(|child| {
                search_radius(child, point, radius, neighbors);
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
