use crate::SRTree;
use crate::{measure::distance::Metric, shape::point::Point};
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

impl<T, M> SRTree<T, M>
where
    T: Float + Send + Sync,
    M: Metric<T>,
{
    pub fn query(&self, point_coords: &[T], k: usize) -> (Vec<usize>, Vec<T>) {
        let mut neighbors = BinaryHeap::new();
        self.search(
            &Point::with_coords(point_coords.to_vec()),
            self.root_index,
            k,
            &mut neighbors,
        );
        let neighbors = neighbors.into_sorted_vec();

        let indices = neighbors.iter().map(|n| n.point_index).collect();
        let distances = neighbors.iter().map(|n| n.distance.into_inner()).collect();
        (indices, distances)
    }

    fn search(
        &self,
        point: &Point<T>,
        node_index: usize,
        k: usize,
        neighbors: &mut BinaryHeap<Neighbor<T>>,
    ) {
        let node = &self.nodes[node_index];

        let mut kth_distance = OrderedFloat(T::infinity());
        if node.is_leaf() {
            let distance_to_center = self.distance(point, &node.sphere.center);
            for candidate_index in node.points() {
                let candidate = &self.points[*candidate_index];
                if neighbors.len() == k {
                    kth_distance = neighbors.peek().unwrap().distance;
                }

                // ball-bound pruning
                let ball_bound = (distance_to_center - candidate.radius).max(T::zero());
                if OrderedFloat(ball_bound) > kth_distance {
                    break;
                }

                let neighbor_distance = OrderedFloat(self.distance(point, candidate));
                if neighbors.len() < k {
                    neighbors.push(Neighbor::new(neighbor_distance, candidate.index));
                } else if neighbor_distance < kth_distance {
                    neighbors.pop();
                    neighbors.push(Neighbor::new(neighbor_distance, candidate.index));
                }
            }
        } else {
            let mut to_visit = Vec::new();
            for child_index in node.nodes() {
                let child = &self.nodes[*child_index];
                let distance = OrderedFloat(self.point_to_node_min_distance(point, child));
                to_visit.push((distance, *child_index));
            }
            to_visit.sort();

            for (child_distance, child_index) in to_visit {
                // if k neighbors were already sampled, then the target distance is kth closest distance:
                if neighbors.len() == k {
                    kth_distance = neighbors.peek().unwrap().distance;
                }

                // search pruning: don't visit nodes with min_distance bigger than kth distance
                if child_distance > kth_distance {
                    break;
                }

                self.search(point, child_index, k, neighbors);
            }
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
        let tree = SRTree::euclidean_with_params(&points, Params::new(2, 5).unwrap())
            .expect("Failed to build SRTree");
        let (indices, distances) = tree.query(&[0.0, 0.0], 3);
        assert_eq!(indices, vec![0, 1, 2]);
        assert_eq!(distances, vec![0.0, 2_f64.sqrt(), 8_f64.sqrt()]);
    }
}
