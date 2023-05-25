use crate::SRTree;
use ordered_float::Float;
use std::{
    fmt::Debug,
    ops::{AddAssign, DivAssign, MulAssign, SubAssign},
};

impl<T> SRTree<T>
where
    T: Float + Send + Sync,
{
    #[must_use]
    pub fn calculate_variance(&self, node_index: usize) -> Vec<T>
    where
        T: Debug + Float + AddAssign + SubAssign + MulAssign + DivAssign,
    {
        if self.nodes[node_index].is_leaf() {
            self.calculate_leaf_variance(node_index)
        } else {
            self.calculate_node_variance(node_index)
        }
    }

    fn calculate_leaf_variance(&self, leaf_index: usize) -> Vec<T> {
        let leaf = &self.nodes[leaf_index];

        // 1. Calculate mean (mean of entries)
        let mean = self.calculate_mean(leaf_index);

        // 2. Calculate variance w.r.t. the mean
        let mut variance = vec![T::zero(); self.dimension];
        for i in 0..leaf.immed_children() {
            let point_index = leaf.points()[i];
            let point = &self.points[point_index];
            for axis_index in 0..variance.len() {
                variance[axis_index] =
                    variance[axis_index] + (point.coords[axis_index] - mean[axis_index]).powi(2);
            }
        }

        let number_of_points = T::from(leaf.immed_children()).unwrap();
        for var in &mut variance {
            *var = *var / number_of_points;
        }
        variance
    }

    fn calculate_node_variance(&self, node_index: usize) -> Vec<T> {
        let node = &self.nodes[node_index];

        // 1. Calculate mean (mean of entries)
        let mean = self.calculate_mean(node_index);

        // 2. Calculate variance w.r.t. the mean
        let mut number_of_entries = T::zero();
        let mut variance = vec![T::zero(); self.dimension];
        for i in 0..node.immed_children() {
            let child_index = node.nodes()[i];
            let child = &self.nodes[child_index];
            let child_number_of_entries = T::from(child.immed_children()).unwrap();
            for axis_index in 0..variance.len() {
                variance[axis_index] = variance[axis_index]
                    + (child.sphere.center.coords[axis_index] - mean[axis_index]).powi(2)
                        * child_number_of_entries;
                if !node.is_leaf() {
                    variance[axis_index] =
                        variance[axis_index] + child_number_of_entries * child.variance[axis_index];
                }
            }
            number_of_entries = number_of_entries + child_number_of_entries;
        }
        variance
    }
}
