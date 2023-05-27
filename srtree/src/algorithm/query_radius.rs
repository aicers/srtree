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

#[cfg(test)]
mod tests {
    use crate::{Params, SRTree};

    #[test]
    pub fn test_query() {
        let points = vec![
            vec![0.0, 0.0],
            vec![1.0, 1.0],
            vec![2.0, 2.0],
            vec![3.0, 3.0],
            vec![4.0, 4.0],
            vec![5.0, 5.0],
            vec![6.0, 6.0],
            vec![7.0, 7.0],
            vec![8.0, 8.0],
            vec![9.0, 9.0],
        ];
        let tree = SRTree::new_with_params(&points, Params::new(2, 5).unwrap())
            .expect("Failed to build SRTree");
        let mut indices = tree.query_radius(&[0.0, 0.0], 8_f64.sqrt());
        indices.sort();
        assert_eq!(indices, vec![0, 1, 2]);
    }
}
