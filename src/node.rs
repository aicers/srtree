use crate::{
    measure::distance::euclidean,
    shape::{rect::Rect, sphere::Sphere},
};
use ordered_float::{Float, OrderedFloat};
use std::{
    ops::{AddAssign, DivAssign, MulAssign, SubAssign},
    rc::Weak, fmt::Debug,
};

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
    T: Debug + Copy + Float + AddAssign + SubAssign + MulAssign + DivAssign,
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
            0,
            self.height,
        )
    }

    pub fn new_point(point: &Vec<T>) -> Node<T> {
        Node::new(
            Rect::from_point(point),
            Sphere::from_point(point),
            Data::Points(Vec::with_capacity(1)),
            vec![T::zero(); point.len()],
            0,
            0,
        )
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

    pub fn pop_last(&mut self, n: usize) -> Vec<Node<T>> {
        let center = self.get_sphere().center.clone();
        let number_of_immediate_children = self.immed_children();
        if self.is_leaf() {
            self.points_mut()
                .sort_by_key(|p| OrderedFloat(euclidean(&center, p)));
            self.points_mut()
                .split_off(number_of_immediate_children - n)
                .iter()
                .map(|p| Node::new_point(p))
                .collect()
        } else {
            self.nodes_mut()
                .sort_by_key(|node| OrderedFloat(euclidean(&center, &node.sphere.center)));
            self.nodes_mut().split_off(number_of_immediate_children - n)
        }
    }

    pub fn intersects_point(&self, point: &Vec<T>) -> bool {
        self.rect.intersects_point(point) && self.sphere.intersects_point(point)
    }
}

#[cfg(test)]
mod tests {

    use crate::{algorithm::insertion::insert_data, shape::reshape::reshape};

    use super::*;

    #[test]
    pub fn test_pop_last() {
        let origin = vec![0., 0.];
        let mut leaf_node = Node::new_leaf(&origin, 10);
        leaf_node.points_mut().push(origin);
        for i in 1..10 {
            leaf_node.points_mut().push(vec![0., i as f64]);
        }
        reshape(&mut leaf_node);
        assert_eq!(leaf_node.get_sphere().center, vec![0., 4.5]);
        let last = leaf_node.pop_last(2);
        assert_eq!(last[0].get_sphere().center, vec![0., 0.]);
        assert_eq!(last[1].get_sphere().center, vec![0., 9.]);
    }
}
