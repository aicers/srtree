use crate::shape::{point::Point, rect::Rect, sphere::Sphere};
use ordered_float::Float;

pub enum Data {
    Points(Vec<usize>),
    Nodes(Vec<usize>),
}

pub struct Node<T> {
    pub rect: Rect<T>,
    pub sphere: Sphere<T>,
    data: Data,
    pub height: usize,
    pub parent_index: usize,
}

impl<T> Node<T>
where
    T: Float + Send + Sync,
{
    pub fn new(rect: Rect<T>, sphere: Sphere<T>, data: Data, height: usize) -> Node<T> {
        Node {
            rect,
            sphere,
            data,
            height,
            parent_index: usize::MAX,
        }
    }

    pub fn new_node(children_indices: Vec<usize>, height: usize) -> Node<T> {
        Node::new(
            Rect::from_point(&Point::with_coords(Vec::new())),
            Sphere::from_point(&Point::with_coords(Vec::new())),
            Data::Nodes(children_indices),
            height,
        )
    }

    pub fn new_leaf(points_indices: Vec<usize>) -> Node<T> {
        Node::new(
            Rect::from_point(&Point::with_coords(Vec::new())),
            Sphere::from_point(&Point::with_coords(Vec::new())),
            Data::Points(points_indices),
            1,
        )
    }

    pub fn is_leaf(&self) -> bool {
        matches!(self.data, Data::Points(_))
    }

    pub fn children(&self) -> &Vec<usize> {
        match &self.data {
            Data::Nodes(nodes) => nodes,
            Data::Points(_) => panic!("not a node"),
        }
    }

    pub fn points(&self) -> &Vec<usize> {
        match &self.data {
            Data::Points(points) => points,
            Data::Nodes(_) => panic!("not a leaf"),
        }
    }

    pub fn points_mut(&mut self) -> &mut Vec<usize> {
        match &mut self.data {
            Data::Points(points) => points,
            Data::Nodes(_) => panic!("not a leaf"),
        }
    }

    pub fn set_points(&mut self, points: Vec<usize>) {
        if self.is_leaf() {
            self.data = Data::Points(points);
        }
    }

    pub fn immed_children(&self) -> usize {
        match &self.data {
            Data::Points(pts) => pts.len(),
            Data::Nodes(children) => children.len(),
        }
    }
}
