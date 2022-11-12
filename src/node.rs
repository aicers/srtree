use crate::distance::distance;
use crate::rect::Rect;
use crate::sphere::Sphere;
use ordered_float::{Float, OrderedFloat};
use priority_queue::PriorityQueue;
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
}

#[allow(dead_code)]
impl<T> Node<T>
where
    T: Float + AddAssign + SubAssign + MulAssign + DivAssign,
{
    pub fn new(rect: Rect<T>, sphere: Sphere<T>, data: Data<T>, total_children: usize) -> Node<T> {
        Node {
            rect,
            sphere,
            data,
            total_children,
        }
    }

    pub fn new_node(point: &Vec<T>, capacity: usize) -> Node<T> {
        Node::new(
            Rect::from_point(point).unwrap(),
            Sphere::from_point(point),
            Data::Nodes(Vec::with_capacity(capacity)),
            0,
        )
    }

    pub fn new_leaf(point: &Vec<T>, capacity: usize) -> Node<T> {
        Node::new(
            Rect::from_point(point).unwrap(),
            Sphere::from_point(point),
            Data::Points(Vec::with_capacity(capacity)),
            0,
        )
    }

    fn nodes(&mut self) -> &mut Vec<Node<T>> {
        match &mut self.data {
            Data::Nodes(nodes) => nodes,
            Data::Points(_) => panic!("not a node"),
        }
    }

    fn points(&mut self) -> &mut Vec<Vec<T>> {
        match &mut self.data {
            Data::Points(points) => points,
            Data::Nodes(_) => panic!("not a leaf"),
        }
    }

    fn choose_subtree(&mut self, point: &Vec<T>, level: usize) -> &mut Node<T> {
        match &self.data {
            Data::Points(_) => self,
            Data::Nodes(nodes) => {
                let mut closest_distance = T::infinity();
                let mut closest_index: usize = 0;
                for (i, node) in nodes.iter().enumerate() {
                    let current_distance = distance(&node.sphere.center, point);
                    if current_distance <= closest_distance {
                        closest_distance = current_distance;
                        closest_index = i;
                    }
                }
                self.nodes()[closest_index].choose_subtree(point, level + 1)
            }
        }
    }

    fn insert(&mut self, point: &Vec<T>, level: usize) {
        match &self.data {
            Data::Nodes(nodes) => {
                let parent = self.choose_subtree(point, level);
                parent.insert_data(point);
            }
            Data::Points(points) => {
                self.points().push(point.clone());
                self.total_children += 1;
            }
        }
    }

    pub fn insert_data(&mut self, point: &Vec<T>) {
        self.insert(point, 0);
    }

    pub fn intersects_point(&self, point: &Vec<T>) -> bool {
        self.rect.intersects_point(point) && self.sphere.intersects_point(point)
    }

    pub fn query(&self, point: &Vec<T>, k: usize, result: &mut Vec<Vec<T>>) {
        match &self.data {
            Data::Nodes(nodes) => {}
            Data::Points(points) => {
                let mut queue: PriorityQueue<usize, OrderedFloat<T>> = PriorityQueue::new();
                for (index, candidate) in points.iter().enumerate() {
                    queue.push(index, -OrderedFloat(distance(candidate, point)));
                }
                while !queue.is_empty() {
                    let (index, distance) = queue.pop().unwrap();
                    if result.len() < k {
                        result.push(points[index].clone());
                    } else {
                        break;
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_leaf_node() {
        let point = vec![0., 0.];
        let mut leaf_node = Node::new_leaf(&point, 10);
        leaf_node.insert(&point, 0);
        let mut result = Vec::new();
        leaf_node.query(&point, 10, &mut result);
        assert!(result.contains(&point));
    }
}
