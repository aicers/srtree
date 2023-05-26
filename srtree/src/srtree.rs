use crate::node::Node;
use crate::params::Params;
use crate::shape::point::Point;
use ordered_float::Float;

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
    #[must_use]
    pub fn new_with_params(pts: &[Vec<T>], mut params: Params) -> SRTree<T> {
        params.dimension = pts[0].len();
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
        tree
    }

    #[must_use]
    pub fn new(pts: &[Vec<T>]) -> SRTree<T> {
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
