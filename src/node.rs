use crate::shape::{rect::Rect, sphere::Sphere};
use ordered_float::Float;
use std::ops::{AddAssign, DivAssign, MulAssign, SubAssign};

#[allow(dead_code)]
pub enum Data<T> {
    Points(Vec<Vec<T>>),
    Nodes(Vec<Node<T>>),
}

#[allow(dead_code)]
pub struct Node<T> {
    rect: Rect<T>,
    sphere: Sphere<T>,
    data: Data<T>,
    variance: Vec<T>,
    total_children: usize,
    height: usize, // height above leaf
}

#[allow(dead_code)]
impl<T> Node<T>
where
    T: Copy + Float + AddAssign + SubAssign + MulAssign + DivAssign,
{
    pub fn new(
        rect: Rect<T>,
        sphere: Sphere<T>,
        data: Data<T>,
        variance: Vec<T>,
        total_children: usize,
        height: usize,
    ) -> Node<T> {
        Node {
            rect,
            sphere,
            data,
            variance,
            total_children,
            height,
        }
    }

    pub fn new_node(point: &Vec<T>, capacity: usize, height: usize) -> Node<T> {
        Node::new(
            Rect::from_point(point),
            Sphere::from_point(point),
            Data::Nodes(Vec::with_capacity(capacity)),
            vec![T::zero(); point.len()],
            0,
            height,
        )
    }

    pub fn new_leaf(point: &Vec<T>, capacity: usize) -> Node<T> {
        Node::new(
            Rect::from_point(point),
            Sphere::from_point(point),
            Data::Points(Vec::with_capacity(capacity)),
            vec![T::zero(); point.len()],
            0,
            0,
        )
    }

    pub fn new_sibling(node: &Node<T>, capacity: usize) -> Node<T> {
        let data = match node.data {
            Data::Nodes(_) => Data::Nodes(Vec::with_capacity(capacity)),
            Data::Points(_) => Data::Points(Vec::with_capacity(capacity)),
        };
        Node::new(
            Rect::from_point(&node.get_sphere().center),
            Sphere::from_point(&node.get_sphere().center),
            data,
            vec![T::zero(); node.dimension()],
            0,
            0,
        )
    }

    pub fn new_point(point: &Vec<T>) -> Node<T> {
        Node::new_leaf(point, 1)
    }

    pub fn is_leaf(&self) -> bool {
        matches!(self.data, Data::Points(_))
    }

    pub fn dimension(&self) -> usize {
        self.sphere.center.len()
    }

    pub fn set_rect(&mut self, rect: Rect<T>) {
        self.rect = rect;
    }

    pub fn get_rect(&self) -> &Rect<T> {
        &self.rect
    }

    pub fn set_sphere(&mut self, sphere: Sphere<T>) {
        self.sphere = sphere;
    }

    pub fn get_sphere(&self) -> &Sphere<T> {
        &self.sphere
    }

    pub fn set_variance(&mut self, variance: Vec<T>) {
        self.variance = variance;
    }

    pub fn get_variance(&self) -> &Vec<T> {
        &self.variance
    }

    pub fn nodes(&self) -> &Vec<Node<T>> {
        match &self.data {
            Data::Nodes(nodes) => nodes,
            Data::Points(_) => panic!("not a node"),
        }
    }

    pub fn nodes_mut(&mut self) -> &mut Vec<Node<T>> {
        match &mut self.data {
            Data::Nodes(nodes) => nodes,
            Data::Points(_) => panic!("not a node"),
        }
    }

    pub fn points(&self) -> &Vec<Vec<T>> {
        match &self.data {
            Data::Points(points) => points,
            Data::Nodes(_) => panic!("not a leaf"),
        }
    }

    pub fn points_mut(&mut self) -> &mut Vec<Vec<T>> {
        match &mut self.data {
            Data::Points(points) => points,
            Data::Nodes(_) => panic!("not a leaf"),
        }
    }

    pub fn immed_children(&self) -> usize {
        match &self.data {
            Data::Nodes(_) => self.nodes().len(),
            Data::Points(_) => self.points().len(),
        }
    }

    pub fn child_centroid(&self, i: usize) -> &Vec<T> {
        match &self.data {
            Data::Nodes(_) => &self.nodes()[i].sphere.center,
            Data::Points(_) => &self.points()[i],
        }
    }

    pub fn child_immed_children(&self, i: usize) -> usize {
        match &self.data {
            Data::Nodes(_) => self.nodes()[i].immed_children(),
            Data::Points(_) => 1,
        }
    }

    pub fn child_variance(&self, i: usize) -> &Vec<T> {
        match &self.data {
            Data::Nodes(_) => self.nodes()[i].get_variance(),
            Data::Points(_) => {
                panic!("Trying to access variance of a point");
            }
        }
    }

    pub fn set_height(&mut self, height: usize) {
        self.height = height;
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn intersects_point(&self, point: &Vec<T>) -> bool {
        self.rect.intersects_point(point) && self.sphere.intersects_point(point)
    }
}
