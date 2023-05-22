use crate::shape::{point::Point, rect::Rect, reshape::reshape, sphere::Sphere};
use ordered_float::{Float, OrderedFloat};

pub enum Data<T> {
    Points(Vec<Point<T>>),
    Nodes(Vec<Node<T>>),
}

pub struct Node<T> {
    pub rect: Rect<T>,
    pub sphere: Sphere<T>,
    data: Data<T>,
    pub variance: Vec<T>,
    pub height: usize,
}

impl<T> Node<T>
where
    T: Float + Send + Sync,
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
            Rect::from_point(&self.sphere.center),
            Sphere::from_point(&self.sphere.center),
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
            &nodes[0].sphere.center,
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
            Data::Nodes(_) => &self.nodes()[i].variance,
            Data::Points(_) => {
                panic!("Trying to access variance of a point");
            }
        }
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn pop_farthest(&mut self, n: usize) -> Vec<Node<T>> {
        let center = self.sphere.center.clone();
        let number_of_immediate_children = self.immed_children();
        if self.is_leaf() {
            self.points_mut().select_nth_unstable_by(n, |a, b| {
                OrderedFloat(center.distance(a)).cmp(&OrderedFloat(center.distance(b)))
            });
            self.points_mut()
                .split_off(number_of_immediate_children - n)
                .iter()
                .map(|p| Node::new_point(p))
                .collect()
        } else {
            self.nodes_mut().select_nth_unstable_by(n, |a, b| {
                OrderedFloat(center.distance(&a.sphere.center))
                    .cmp(&OrderedFloat(center.distance(&b.sphere.center)))
            });
            self.nodes_mut().split_off(number_of_immediate_children - n)
        }
    }

    pub fn min_distance(&self, point: &Point<T>) -> T
    where
        T: Float + Send + Sync,
    {
        let ds = self.sphere.min_distance(point);
        let dr = self.rect.min_distance(point);
        ds.max(dr)
    }

    pub fn max_distance(&self, point: &Point<T>) -> T
    where
        T: Float + Send + Sync,
    {
        let ds = self.sphere.max_distance(point);
        let dr = self.rect.max_distance(point);
        ds.min(dr)
    }

    pub fn node_count(&self) -> usize {
        1 + match &self.data {
            Data::Nodes(nodes) => nodes.iter().map(Node::node_count).sum(),
            Data::Points(_) => 0,
        }
    }

    pub fn leaf_count(&self) -> usize {
        match &self.data {
            Data::Nodes(nodes) => nodes.iter().map(Node::leaf_count).sum(),
            Data::Points(_) => 1,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::shape::reshape::reshape;

    #[test]
    pub fn test_pop_farthest_children() {
        let origin = Point::with_coords(vec![0., 0.]);
        let mut leaf_node = Node::new_leaf(&origin, 10);
        leaf_node.points_mut().push(origin);
        for i in 1..10 {
            leaf_node
                .points_mut()
                .push(Point::with_coords(vec![0., i as f64]));
        }
        reshape(&mut leaf_node);
        assert_eq!(leaf_node.sphere.center.coords, vec![0., 4.5]);
        let last = leaf_node.pop_farthest(2);
        assert_eq!(last[0].sphere.center.coords, vec![0., 0.]);
        assert_eq!(last[1].sphere.center.coords, vec![0., 9.]);
    }

    #[test]
    pub fn test_node_count() {
        let points1 = vec![
            Point::with_coords(vec![0., 0.]),
            Point::with_coords(vec![0., 1.]),
        ];
        let leaf1 = Node::create_leaf(points1);
        let points2 = vec![
            Point::with_coords(vec![10., 0.]),
            Point::with_coords(vec![10., 1.]),
        ];
        let leaf2 = Node::create_leaf(points2);
        let parent = Node::create_parent(vec![leaf1, leaf2]);
        assert_eq!(parent.leaf_count(), 2);
        assert_eq!(parent.node_count(), 3);
    }
}
