use crate::{
    node::Node,
    shape::{point::Point, rect::Rect, sphere::Sphere},
    SRTree,
};
use ordered_float::Float;

pub trait Metric<T> {
    fn distance(&self, point1: &[T], point2: &[T]) -> T;
    fn distance_squared(&self, point1: &[T], point2: &[T]) -> T;
}

#[derive(Default, Clone)]
pub struct Euclidean {}

impl<T> Metric<T> for Euclidean
where
    T: Float + Send + Sync,
{
    fn distance(&self, point1: &[T], point2: &[T]) -> T {
        self.distance_squared(point1, point2).sqrt()
    }

    fn distance_squared(&self, point1: &[T], point2: &[T]) -> T {
        if point1.len() != point2.len() {
            return T::infinity();
        }
        let mut distance = T::zero();
        for i in 0..point1.len() {
            distance = distance + (point1[i] - point2[i]).powi(2);
        }
        distance
    }
}

impl<T, M> SRTree<T, M>
where
    T: Float + Send + Sync,
    M: Metric<T>,
{
    pub fn distance(&self, a: &Point<T>, b: &Point<T>) -> T {
        self.metric.distance(&a.coords, &b.coords)
    }

    pub fn distance_squared(&self, a: &Point<T>, b: &Point<T>) -> T {
        self.metric.distance_squared(&a.coords, &b.coords)
    }

    pub fn point_to_rect_min_distance(&self, point: &Point<T>, rect: &Rect<T>) -> T {
        let closest_point = rect.closest_point_to(point);
        self.distance(point, &closest_point)
    }

    pub fn point_to_rect_max_distance(&self, point: &Point<T>, rect: &Rect<T>) -> T {
        let farthest_point = rect.farthest_point_to(point);
        self.distance(point, &farthest_point)
    }

    pub fn point_to_sphere_min_distance(&self, point: &Point<T>, sphere: &Sphere<T>) -> T {
        let distance = self.distance(point, &sphere.center);
        T::zero().max(distance - sphere.radius)
    }

    pub fn point_to_sphere_max_distance(&self, point: &Point<T>, sphere: &Sphere<T>) -> T {
        let distance = self.distance(point, &sphere.center);
        distance + sphere.radius
    }

    pub fn point_to_node_min_distance(&self, point: &Point<T>, node: &Node<T>) -> T {
        let ds = self.point_to_sphere_min_distance(point, &node.sphere);
        let dr = self.point_to_rect_min_distance(point, &node.rect);
        ds.max(dr)
    }

    pub fn point_to_node_max_distance(&self, point: &Point<T>, node: &Node<T>) -> T {
        let ds = self.point_to_sphere_max_distance(point, &node.sphere);
        let dr = self.point_to_rect_max_distance(point, &node.rect);
        ds.max(dr)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_dimension_mismatch() {
        let point1 = vec![1., 0., 0.];
        let point2 = vec![2., 0.];

        let euclidean = Euclidean::default();
        assert_eq!(euclidean.distance(&point1, &point2), f32::INFINITY);
    }

    #[test]
    pub fn test_euclidean() {
        let point1 = vec![1., 0., 0.];
        let point2 = vec![2., 0., 0.];

        let euclidean = Euclidean::default();
        assert_eq!(euclidean.distance(&point1, &point2), 1.);
    }
}
