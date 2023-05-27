use crate::node::Node;
use crate::params::Params;
use crate::shape::point::Point;
use ordered_float::Float;

#[derive(Debug)]
pub enum ArrayError {
    Empty,
    DimensionMismatch,
}

pub struct SRTree<T> {
    pub root_index: usize,
    pub points: Vec<Point<T>>,
    pub nodes: Vec<Node<T>>,
    pub params: Params,
}

impl<T> SRTree<T>
where
    T: Float + Send + Sync,
{
    /// Builds `SRTree` with the given points and params.
    /// 
    /// # Errors
    /// * `ArrayError::Empty` if the input array is empty.
    /// * `ArrayError::DimensionMismatch` if the input array contains points of different dimensions.
    pub fn new_with_params(pts: &[Vec<T>], mut params: Params) -> Result<Self, ArrayError> {
        if pts.is_empty() {
            return Err(ArrayError::Empty);
        }
        params.dimension = pts[0].len();
        if !pts.iter().all(|p| p.len() == params.dimension) {
            return Err(ArrayError::DimensionMismatch);
        }

        let points: Vec<Point<T>> = pts
            .iter()
            .enumerate()
            .map(|(i, p)| Point::new(p.clone(), i))
            .collect();
        let point_indices = (0..points.len()).collect();
        let mut tree = SRTree {
            root_index: usize::MAX,
            points,
            nodes: Vec::new(),
            params,
        };
        tree.root_index = tree.bulk_load(point_indices);
        Ok(tree)
    }

    /// Builds `SRTree` with the given points (using default params).
    /// 
    /// # Errors
    /// * `ArrayError::Empty` if the input array is empty.
    /// * `ArrayError::DimensionMismatch` if the input array contains points of different dimensions.
    pub fn new(pts: &[Vec<T>]) -> Result<Self, ArrayError> {
        SRTree::new_with_params(pts, Params::default_params())
    }

    pub fn add_node(&mut self, node: Node<T>) -> usize {
        let index = self.nodes.len();
        self.nodes.push(node);
        index
    }

    #[must_use]
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    #[must_use]
    pub fn leaf_count(&self) -> usize {
        self.nodes.iter().filter(|n| n.is_leaf()).count()
    }

    #[must_use]
    pub fn height(&self) -> usize {
        self.nodes[self.root_index].height
    }
}

#[cfg(test)]
mod tests {
    use crate::SRTree;

    #[test]
    pub fn test_empty_input() {
        let pts: Vec<Vec<f64>> = Vec::new();
        let tree = SRTree::new(&pts);
        assert!(tree.is_err());
    }

    #[test]
    pub fn test_dimension_mismatch() {
        let pts = vec![vec![1.0, 2.0], vec![3.0]];
        let tree = SRTree::new(&pts);
        assert!(tree.is_err());
    }

    #[test]
    pub fn test_valid_input() {
        let pts = vec![vec![1.0, 2.0]];
        let tree = SRTree::new(&pts);
        assert!(tree.is_ok());
    }

    #[test]
    pub fn test_large_input() {
        let mut pts = Vec::new();
        for i in 0..1000 {
            pts.push(vec![i as f64, i as f64]);
        }
        let tree = SRTree::new(&pts);
        assert!(tree.is_ok());
    }

    #[test]
    pub fn test_high_dimension() {
        let dim = 100;
        let mut pts = Vec::new();
        for i in 0..100 {
            let mut pt = Vec::new();
            for _ in 0..dim {
                pt.push(i as f64);
            }
            pts.push(pt);
        }
        let tree = SRTree::new(&pts);
        assert!(tree.is_ok());
    }
}
