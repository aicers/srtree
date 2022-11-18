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
    total_children: usize,
    height: usize, // height above leaf
    dimension: usize,
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
        total_children: usize,
        height: usize,
        dimension: usize,
    ) -> Node<T> {
        Node {
            rect,
            sphere,
            data,
            total_children,
            height,
            dimension,
        }
    }

    pub fn new_node(point: &Vec<T>, capacity: usize, height: usize) -> Node<T> {
        Node::new(
            Rect::from_point(point).unwrap(),
            Sphere::from_point(point),
            Data::Nodes(Vec::with_capacity(capacity)),
            0,
            height,
            point.len(),
        )
    }

    pub fn new_leaf(point: &Vec<T>, capacity: usize) -> Node<T> {
        Node::new(
            Rect::from_point(point).unwrap(),
            Sphere::from_point(point),
            Data::Points(Vec::with_capacity(capacity)),
            0,
            0,
            point.len(),
        )
    }

    pub fn sphere(&self) -> &Sphere<T> {
        &self.sphere
    }

    pub fn rectangle(&self) -> &Rect<T> {
        &self.rect
    }

    pub fn is_leaf(&self) -> bool {
        matches!(self.data, Data::Points(_))
    }

    pub fn dimension(&self) -> usize {
        self.dimension
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

    pub fn intersects_point(&self, point: &Vec<T>) -> bool {
        self.rect.intersects_point(point) && self.sphere.intersects_point(point)
    }
}
