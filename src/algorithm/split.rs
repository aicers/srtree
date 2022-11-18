use crate::node::Node;
use ordered_float::Float;
use std::ops::{AddAssign, DivAssign, MulAssign, SubAssign};

fn calculate_mean<T>(points: &Vec<Vec<T>>) -> Vec<T>
where T: Float + AddAssign + SubAssign + MulAssign + DivAssign,
{
    if points.is_empty() {
        return Vec::new();
    }
    let mut number_of_points = T::zero();
    let mut mean: Vec<T> = vec![T::zero(); points[0].len()];
    points.iter().for_each(|point| {
        for i in 0..point.len() {
            mean[i] += point[i];
        }
        number_of_points += T::one();
    });
    for i in 0..mean.len() {
        mean[i] /= number_of_points;
    }
    mean
}

fn calculate_variance<T>(points: &Vec<Vec<T>>, mean: &Vec<T>) -> Vec<T>
where T: Float + AddAssign + SubAssign + MulAssign + DivAssign,
{
    if points.is_empty() {
        return Vec::new();
    }
    let mut number_of_points = T::zero();
    let mut variance = vec![T::zero(); points[0].len()];
    points.iter().for_each(|point| {
        for i in 0..point.len() {
            variance[i] += (point[i] - mean[i]).powi(2);
        }
        number_of_points += T::one();
    });
    for i in 0..mean.len() {
        variance[i] /= number_of_points;
    }
    variance
}

fn choose_split_dimension<T>(node: &Node<T>) -> usize
where T: Float + AddAssign + SubAssign + MulAssign + DivAssign,
{
    // 1. Calculate mean for each dimension:
    let mean = calculate_mean(node.points());

    // 2. Find the sum of squared difference for each dimension and divide by number_of_points:
    let variance = calculate_variance(node.points(), &mean);

    // 3. Choose the dimension with the highest variance
    let mut selected_index = 0;
    for i in 0..variance.len() {
        if variance[i] > variance[selected_index] {
            selected_index = i;
        }
    }
    selected_index
}

fn choose_split_index<T>(node: &Node<T>, dimension: usize) -> usize {
    0
}

pub fn split<'a, T>(node: &'a mut Node<T>) -> Option<Node<T>> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    pub fn get_test_points() -> Vec<Vec<f64>> {
        let mut points = Vec::new();
        points.push(vec![0., 0.]);
        points.push(vec![0., 1.]);
        points.push(vec![0., 2.]);
        points.push(vec![0., 8.]);
        points.push(vec![0., 9.]);
        points
    }

    #[test]
    pub fn test_mean_calculation(){
        let points = get_test_points();
        let mean = calculate_mean(&points);
        assert_eq!(mean[0], 0.);
        assert_eq!(mean[1], 4.);
    }

    #[test]
    pub fn test_mean_variance(){
        let points = get_test_points();
        let mean = calculate_mean(&points);
        let variance = calculate_variance(&points, &mean);
        assert_eq!(variance[0], 0.);
        assert_eq!(variance[1], 14.);
    }

    #[test]
    pub fn test_choose_split_dimension() {
        let origin = vec![0., 0.];
        let mut node = Node::new_leaf(&origin, 10);
        get_test_points().iter().for_each(|point| {
            node.points_mut().push(point.to_owned());
        });

        let expected_dimension = 1;
        let selected_dimension = choose_split_dimension(&node);
        assert_eq!(expected_dimension, selected_dimension);
    }

    #[test]
    pub fn test_choose_split_index() {
        let origin = vec![0., 0.];
        let mut node = Node::new_leaf(&origin, 10);
        get_test_points().iter().for_each(|point| {
            node.points_mut().push(point.to_owned());
        });

        let expected_index = 3;
        let selected_index = choose_split_index(&node, 1);
        assert_eq!(expected_index, selected_index);
    }
}
