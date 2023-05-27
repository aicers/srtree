use super::{point::Point, rect::Rect, sphere::Sphere};
use crate::SRTree;
use ordered_float::{Float, OrderedFloat};

impl<T> SRTree<T>
where
    T: Float + Send + Sync,
{
    pub fn reshape(&mut self, node_index: usize) {
        let centroid = Point::new(self.calculate_mean(node_index), node_index);
        let node = &self.nodes[node_index];

        let mut max_distance = T::zero();
        let mut low = centroid.coords.clone();
        let mut high = centroid.coords.clone();
        if node.is_leaf() {
            let mut points = Vec::with_capacity(node.points().len());
            for point_index in node.points() {
                let point = &self.points[*point_index];
                for i in 0..low.len() {
                    low[i] = low[i].min(point.coords[i]);
                    high[i] = high[i].max(point.coords[i]);
                }
                let distance_to_point = centroid.distance(point);
                max_distance = max_distance.max(distance_to_point);
                points.push((distance_to_point, *point_index));
            }

            for (distance, point_index) in &points {
                self.points[*point_index].radius = *distance;
                self.points[*point_index].parent_index = node_index;
            }

            points.sort_by_key(|(distance, _)| -OrderedFloat(*distance));
            let points: Vec<usize> = points.into_iter().map(|(_, index)| index).collect();
            self.nodes[node_index].set_points(points);
        } else {
            node.nodes().iter().for_each(|child_index| {
                let child = &self.nodes[*child_index];
                for i in 0..self.params.dimension {
                    low[i] = low[i].min(child.rect.low[i]);
                    high[i] = high[i].max(child.rect.high[i]);
                }
                let distance = child.max_distance(&centroid);
                max_distance = max_distance.max(distance);
            });
        }

        let node = &mut self.nodes[node_index];
        node.rect = Rect::new(low, high);
        node.sphere = Sphere::new(centroid, max_distance);
    }
}

#[cfg(test)]
mod tests {
    use num_traits::Float;

    use crate::SRTree;

    #[test]
    pub fn test_reshape() {
        let pts = vec![
            vec![1.0, 1.0],
            vec![2.0, 2.0],
            vec![3.0, 3.0],
            vec![4.0, 4.0],
            vec![5.0, 5.0],
        ];
        let tree = SRTree::new(&pts).unwrap();
        assert_eq!(tree.nodes[0].rect.low, vec![1., 1.]);
        assert_eq!(tree.nodes[0].rect.high, vec![5., 5.]);
        assert_eq!(tree.nodes[0].sphere.center.coords, vec![3., 3.]);
        assert_eq!(tree.nodes[0].sphere.radius, (4. + 4.).sqrt());
    }
}
