use crate::{
    shape::{point::Point, rect::Rect, reshape::reshape, sphere::Sphere},
};
use ordered_float::{Float, OrderedFloat};
use std::{
    fmt::Debug,
    ops::{AddAssign, DivAssign, MulAssign, SubAssign},
};

pub enum Data<T> {
    Points(Vec<Point<T>>),
    Nodes(Vec<Node<T>>),
}

pub struct Node<T> {
    rect: Rect<T>,
    sphere: Sphere<T>,
    data: Data<T>,
    variance: Vec<T>,
    height: usize, // height above leaf
}

impl<T> Node<T>
where
    T: Debug + Copy + Float + AddAssign + SubAssign + MulAssign + DivAssign,
{
    pub fn new(
        rect: Rect<T>,
        sphere: Sphere<T>,
        data: Data<T>,
        variance: Vec<T>,
        height: usize,
    ) -> Node<T> {
        Node {
            rect,
            sphere,
            data,
            variance,
            height,
        }
    }

    pub fn new_node(point: &Point<T>, capacity: usize, height: usize) -> Node<T> {
        Node::new(
            Rect::from_point(point),
            Sphere::from_point(point),
            Data::Nodes(Vec::with_capacity(capacity)),
            vec![T::zero(); point.dimension()],
            height,
        )
    }

    pub fn new_leaf(point: &Point<T>, capacity: usize) -> Node<T> {
        Node::new(
            Rect::from_point(point),
            Sphere::from_point(point),
            Data::Points(Vec::with_capacity(capacity)),
            vec![T::zero(); point.dimension()],
            1,
        )
    }

    pub fn new_sibling(&self, capacity: usize) -> Node<T> {
        let data = match self.data {
            Data::Nodes(_) => Data::Nodes(Vec::with_capacity(capacity)),
            Data::Points(_) => Data::Points(Vec::with_capacity(capacity)),
        };
        Node::new(
            Rect::from_point(&self.get_sphere().center),
            Sphere::from_point(&self.get_sphere().center),
            data,
            vec![T::zero(); self.dimension()],
            self.height,
        )
    }

    pub fn new_point(point: &Point<T>) -> Node<T> {
        Node::new(
            Rect::from_point(point),
            Sphere::from_point(point),
            Data::Points(Vec::with_capacity(1)),
            vec![T::zero(); point.dimension()],
            0,
        )
    }

    pub fn create_leaf(points: Vec<Point<T>>) -> Node<T> {
        let mut node = Node::new_leaf(&points[0], points.len());
        node.points_mut().extend(points);
        reshape(&mut node);
        node
    }

    pub fn create_parent(nodes: Vec<Node<T>>) -> Node<T> {
        let mut parent = Node::new_node(
            &nodes[0].get_sphere().center,
            nodes.len(),
            nodes[0].get_height() + 1,
        );
        parent.nodes_mut().extend(nodes);
        reshape(&mut parent);
        parent
    }

    pub fn is_leaf(&self) -> bool {
        matches!(self.data, Data::Points(_))
    }

    pub fn dimension(&self) -> usize {
        self.sphere.center.dimension()
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

    pub fn get_variance(&self) -> &[T] {
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

    pub fn points(&self) -> &Vec<Point<T>> {
        match &self.data {
            Data::Points(points) => points,
            Data::Nodes(_) => panic!("not a leaf"),
        }
    }

    pub fn points_mut(&mut self) -> &mut Vec<Point<T>> {
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

    pub fn child_centroid(&self, i: usize) -> &Point<T> {
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

    pub fn child_variance(&self, i: usize) -> &[T] {
        match &self.data {
            Data::Nodes(_) => self.nodes()[i].get_variance(),
            Data::Points(_) => {
                panic!("Trying to access variance of a point");
            }
        }
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn pop_last(&mut self, n: usize) -> Vec<Node<T>> {
        let center = self.get_sphere().center.clone();
        let number_of_immediate_children = self.immed_children();
        if self.is_leaf() {
            self.points_mut().select_nth_unstable_by(n, |a, b| {
                OrderedFloat(center.distance(a))
                    .cmp(&OrderedFloat(center.distance(b)))
            });
            self.points_mut()
                .split_off(number_of_immediate_children - n)
                .iter()
                .map(|p| Node::new_point(p))
                .collect()
        } else {
            self.nodes_mut().select_nth_unstable_by(n, |a, b| {
                OrderedFloat(center.distance(&a.get_sphere().center)).cmp(&OrderedFloat(
                    center.distance(&b.get_sphere().center),
                ))
            });
            self.nodes_mut().split_off(number_of_immediate_children - n)
        }
    }

    pub fn min_distance(&self, point: &Point<T>) -> T
    where
        T: Debug + Float + AddAssign + SubAssign + MulAssign + DivAssign,
    {
        let ds = self.get_sphere().min_distance(point);
        let dr = self.get_rect().min_distance(point);
        ds.max(dr)
    }

    pub fn node_count(&self) -> usize {
        1 + match &self.data {
            Data::Nodes(nodes) => nodes.iter().map(|n| n.node_count()).sum(),
            Data::Points(_) => 0,
        }
    }

    pub fn leaf_count(&self) -> usize {
        match &self.data {
            Data::Nodes(nodes) => nodes.iter().map(|n| n.leaf_count()).sum(),
            Data::Points(_) => 1,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::shape::reshape::reshape;

    #[test]
    pub fn test_pop_last() {
        let origin = Point::with_coords(vec![0., 0.]);
        let mut leaf_node = Node::new_leaf(&origin, 10);
        leaf_node.points_mut().push(origin);
        for i in 1..10 {
            leaf_node
                .points_mut()
                .push(Point::with_coords(vec![0., i as f64]));
        }
        reshape(&mut leaf_node);
        assert_eq!(leaf_node.get_sphere().center.coords(), &vec![0., 4.5]);
        let last = leaf_node.pop_last(2);
        assert_eq!(last[0].get_sphere().center.coords(), &vec![0., 0.]);
        assert_eq!(last[1].get_sphere().center.coords(), &vec![0., 9.]);
    }
}
