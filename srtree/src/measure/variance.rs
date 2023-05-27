use crate::SRTree;
use ordered_float::Float;

use super::distance::Metric;

impl<T, M> SRTree<T, M>
where
    T: Float + Send + Sync,
    M: Metric<T>,
{
    #[must_use]
    pub fn calculate_points_variance(&self, point_indices: &[usize]) -> Vec<T> {
        let dimension = self.params.dimension;
        let mut variances = Vec::new();
        for dim in 0..dimension {
            let variance = self.calculate_dimension_variance(point_indices, dim);
            variances.push(variance);
        }
        variances
    }

    fn calculate_dimension_variance(&self, point_indices: &[usize], dim: usize) -> T {
        let mut sum = T::zero();
        let mut sum_sq = T::zero();
        for point_index in point_indices {
            let coord = self.points[*point_index].coords[dim];
            sum = sum + coord;
            sum_sq = sum_sq + coord * coord;
        }
        let n = T::from(point_indices.len()).unwrap();
        let mean = sum / n;
        let mean_sq = mean * mean;
        sum_sq / n - mean_sq
    }
}

#[cfg(test)]
mod tests {
    use crate::SRTree;

    #[test]
    pub fn test_variance() {
        let points = vec![
            vec![0.0, 0.0],
            vec![1.0, 1.0],
            vec![2.0, 2.0],
            vec![3.0, 3.0],
            vec![4.0, 4.0],
        ];
        let tree = SRTree::euclidean(&points).expect("Failed to build SRTree");
        let variances = tree.calculate_points_variance(&[0, 1, 2, 3, 4]);
        assert_eq!(variances, vec![2.0, 2.0]);
    }
}
