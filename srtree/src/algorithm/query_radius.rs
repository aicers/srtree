use crate::shape::point::Point;
use crate::SRTree;
use ordered_float::Float;

impl<T> SRTree<T>
where
    T: Float + Send + Sync,
{
    pub fn query_radius(&self, point_coords: &[T], radius: T) -> Vec<usize> {
        let mut neighbors = Vec::new();
        self.search_radius(
            self.root_index,
            &Point::new(point_coords.to_vec(), 0),
            radius,
            &mut neighbors,
        );
        neighbors
    }

    fn search_radius(
        &self,
        node_index: usize,
        point: &Point<T>,
        radius: T,
        neighbors: &mut Vec<usize>,
    ) {
        let node = &self.nodes[node_index];
        if node.is_leaf() {
            let distance_to_center = point.distance(&node.sphere.center);
            for candidate_index in node.points() {
                let candidate = &self.points[*candidate_index];

                // ball-bound pruning
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
                .filter(|child| {
                    let child = &self.nodes[**child];
                    child.min_distance(point) <= radius
                })
                .for_each(|child_index| {
                    self.search_radius(*child_index, point, radius, neighbors);
                });
        }
    }
}
