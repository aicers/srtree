use crate::node::Node;
use crate::params::Params;
use crate::shape::point::Point;
use ordered_float::Float;

pub struct SRTree<T> {
    pub root_index: usize,
    pub points: Vec<Point<T>>,
    pub nodes: Vec<Node<T>>,
    pub params: Params,
    pub dimension: usize,
}

impl<T> SRTree<T>
where
    T: Float + Send + Sync,
{
    #[must_use]
    pub fn new_with_params(pts: &[Vec<T>], params: Params) -> SRTree<T> {
        let mut tree = SRTree {
            root_index: usize::MAX,
            points: Vec::new(),
            nodes: Vec::new(),
            params,
            dimension: pts[0].len(),
        };

        let points: Vec<Point<T>> = pts
            .iter()
            .enumerate()
            .map(|(i, p)| Point::new(p.clone(), i))
            .collect();
        tree.points = points.clone();
        tree.root_index = tree.bulk_load(points);
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
