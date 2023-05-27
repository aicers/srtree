use crate::SRTree;
use ordered_float::Float;

impl<T> SRTree<T>
where
    T: Float + Send + Sync,
{
    #[must_use]
    pub fn calculate_mean(&self, node_index: usize) -> Vec<T> {
        if self.nodes[node_index].is_leaf() {
            self.calculate_leaf_mean(node_index)
        } else {
            self.calculate_node_mean(node_index)
        }
    }

    fn calculate_leaf_mean(&self, leaf_index: usize) -> Vec<T> {
        let leaf = &self.nodes[leaf_index];
        let mut mean = vec![T::zero(); self.params.dimension];
        for point_index in leaf.points() {
            let point = &self.points[*point_index];
            for (axis_index, m) in mean.iter_mut().enumerate() {
                *m = *m + point.coords[axis_index];
            }
        }
        let number_of_points = T::from(leaf.immed_children()).unwrap();
        for m in &mut mean {
            *m = *m / number_of_points;
        }
        mean
    }

    fn calculate_node_mean(&self, node_index: usize) -> Vec<T> {
        let root = &self.nodes[node_index];
        let mut number_of_entries = T::zero();
        let mut mean = vec![T::zero(); self.params.dimension];
        for i in 0..root.immed_children() {
            let child_index = root.nodes()[i];
            let child = &self.nodes[child_index];
            let child_number_of_entries = T::from(child.immed_children()).unwrap();

            for (axis_index, m) in mean.iter_mut().enumerate() {
                *m = *m + child.sphere.center.coords[axis_index] * child_number_of_entries;
            }
            number_of_entries = number_of_entries + child_number_of_entries;
        }
        for m in &mut mean {
            *m = *m / number_of_entries;
        }
        mean
    }
}

#[cfg(test)]
mod tests {
    use crate::SRTree;

    #[test]
    pub fn test_mean() {
        let points = vec![
            vec![0.0, 0.0],
            vec![1.0, 1.0],
            vec![2.0, 2.0],
            vec![3.0, 3.0],
            vec![4.0, 4.0],
            vec![5.0, 5.0],
        ];
        let tree = SRTree::new(&points).expect("Failed to build SRTree");
        let mean = tree.calculate_mean(0);
        assert_eq!(mean, vec![2.5, 2.5]);
    }
}
