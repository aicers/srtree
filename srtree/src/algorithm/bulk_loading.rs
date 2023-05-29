use crate::{measure::distance::Metric, node::Node, SRTree};
use num_traits::cast;
use ordered_float::Float;

impl<T, M> SRTree<T, M>
where
    T: Float + Send + Sync,
    M: Metric<T>,
{
    pub fn bulk_load(&mut self, point_indices: Vec<usize>) -> usize {
        if point_indices.is_empty() {
            return usize::MAX;
        }

        if point_indices.len() <= self.params.max_number_of_elements {
            let leaf = Node::new_leaf(point_indices);
            let leaf_index = self.add_node(leaf);
            self.reshape(leaf_index);
            return leaf_index;
        }

        let groups = self.create_entries(point_indices);
        let children: Vec<usize> = groups
            .into_iter()
            .map(|group| self.bulk_load(group))
            .collect();

        let height = 1 + children
            .iter()
            .map(|child_index| self.nodes[*child_index].height)
            .max()
            .unwrap_or(0);

        let root = Node::new_node(children.clone(), height);
        let root_index = self.add_node(root);
        self.reshape(root_index);

        for child_index in children {
            self.nodes[child_index].parent_index = root_index;
        }
        root_index
    }

    fn create_entries(&self, point_indices: Vec<usize>) -> Vec<Vec<usize>> {
        let variances = self.calculate_points_variance(&point_indices);
        let split_dim = variances
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .map(|(i, _)| i)
            .unwrap();
        self.partition_points(point_indices, split_dim)
    }

    fn partition_points(&self, mut point_indices: Vec<usize>, split_dim: usize) -> Vec<Vec<usize>> {
        if point_indices.len() <= self.params.max_number_of_elements {
            return vec![point_indices];
        }
        let partition_size = calculate_internal_node_size(
            point_indices.len(),
            self.params.max_number_of_elements,
            self.params.min_number_of_elements,
        );
        let mut entries = Vec::new();
        while !point_indices.is_empty() {
            let left = point_indices.len().saturating_sub(partition_size);
            point_indices.select_nth_unstable_by(left, |a, b| {
                let (a, b) = (&self.points[*a], &self.points[*b]);
                a.coords[split_dim]
                    .partial_cmp(&b.coords[split_dim])
                    .unwrap()
            });
            let slice = point_indices.split_off(left);
            entries.push(slice);
        }
        entries
    }
}

fn calculate_internal_node_size(n: usize, leaf_size: usize, internal_node_fanout: usize) -> usize {
    if n <= leaf_size {
        return n;
    }

    let n: f64 = cast(n).unwrap();
    let leaf_size: f64 = cast(leaf_size).unwrap();
    let internal_node_fanout: f64 = cast(internal_node_fanout).unwrap();
    if n < 2. * leaf_size {
        return cast(n / 2.).unwrap();
    }

    let num_leaves = n / (2. * leaf_size);
    let num_leaves_per_node = num_leaves.log(internal_node_fanout).floor();
    let internal_node_size = leaf_size * internal_node_fanout.powf(num_leaves_per_node);
    cast(internal_node_size).unwrap()
}

#[cfg(test)]
mod tests {
    use crate::Params;

    use super::*;

    #[test]
    pub fn test_internal_node_size() {
        let n = 5000;
        let leaf_size = 21;
        let internal_node_fanout = 9;
        let internal_node_size = calculate_internal_node_size(n, leaf_size, internal_node_fanout);
        assert_eq!(internal_node_size, 1701);
    }

    #[test]
    pub fn test_bulk_load() {
        let points = vec![
            vec![0., 0.],
            vec![1., 1.],
            vec![2., 2.],
            vec![3., 3.],
            vec![4., 4.],
            vec![5., 5.],
            vec![6., 6.],
            vec![7., 7.],
            vec![8., 8.],
            vec![9., 9.],
        ];
        let tree = SRTree::euclidean_with_params(&points, Params::new(2, 5).unwrap())
            .expect("Failed to build SRTree");
        assert_eq!(tree.nodes.len(), 3);
        assert_eq!(tree.nodes[2].children(), &vec![0, 1]);
        assert_eq!(tree.nodes[2].rect.low, vec![0., 0.]);
        assert_eq!(tree.nodes[2].rect.high, vec![9., 9.]);
        assert_eq!(tree.nodes[1].rect.low, vec![0., 0.]);
        assert_eq!(tree.nodes[1].rect.high, vec![4., 4.]);
        assert_eq!(tree.nodes[0].rect.low, vec![5., 5.]);
        assert_eq!(tree.nodes[0].rect.high, vec![9., 9.]);
    }
}
