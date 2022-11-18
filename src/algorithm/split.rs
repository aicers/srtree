use crate::node::Node;
use ordered_float::{Float, OrderedFloat};
use std::{
    fmt::Debug,
    ops::{AddAssign, DivAssign, MulAssign, SubAssign},
};

fn calculate_mean<T>(points: &Vec<Vec<T>>, from: usize, end: usize) -> Vec<T>
where
    T: Debug + Float + AddAssign + SubAssign + MulAssign + DivAssign,
{
    if points.is_empty() {
        return Vec::new();
    }
    let mut number_of_points = T::zero();
    let mut mean: Vec<T> = vec![T::zero(); points[0].len()];
    for i in from..end {
        for j in 0..points[i].len() {
            mean[j] += points[i][j];
        }
        number_of_points += T::one();
    }
    for i in 0..mean.len() {
        mean[i] /= number_of_points;
    }
    mean
}

fn calculate_variance<T>(points: &Vec<Vec<T>>, from: usize, end: usize) -> Vec<T>
where
    T: Debug + Float + AddAssign + SubAssign + MulAssign + DivAssign,
{
    if points.is_empty() {
        return Vec::new();
    }

    let mean = calculate_mean(points, from, end);
    let mut number_of_points = T::zero();
    let mut variance = vec![T::zero(); mean.len()];

    for i in from..end {
        for j in 0..points[i].len() {
            variance[j] += (points[i][j] - mean[j]).powi(2);
        }
        number_of_points += T::one();
    }
    for i in 0..mean.len() {
        variance[i] /= number_of_points;
    }
    variance
}

fn choose_split_axis<T>(node: &Node<T>) -> usize
where
    T: Debug + Float + AddAssign + SubAssign + MulAssign + DivAssign,
{
    // 1. Calculate variance for each axis:
    let variance = calculate_variance(node.points(), 0, node.points().len());

    // 2. Choose the axis with the highest variance
    let mut selected_index = 0;
    for i in 0..variance.len() {
        if variance[i] > variance[selected_index] {
            selected_index = i;
        }
    }
    selected_index
}

fn choose_split_index<T>(
    node: &mut Node<T>,
    min_number_of_elements: usize,
    max_number_of_elements: usize,
) -> usize
where
    T: Debug + Float + AddAssign + SubAssign + MulAssign + DivAssign,
{
    if node.points().len() < 2 * min_number_of_elements {
        panic!("Cannot split as there're less elements than 2 * min_number_of_elements");
    }

    // 1. Choose the split axis
    let axis = choose_split_axis(node);

    // 2. Sort node points along that axis
    node.points_mut().sort_by_key(|p| OrderedFloat(p[axis]));

    // 3. Minimize the sum of variances for two groups of node.points
    let mut selected_index = min_number_of_elements;
    let mut min_variance = T::infinity();

    let number_of_elements = node.points().len();
    let start = min_number_of_elements;
    let end = max_number_of_elements.min(number_of_elements - min_number_of_elements) + 1;
    for i in start..end {
        let mut current_variance = T::zero();
        calculate_variance(node.points(), 0, i)
            .iter()
            .for_each(|v| {
                current_variance += v.clone();
            });
        calculate_variance(node.points(), i, node.points().len())
            .iter()
            .for_each(|v| {
                current_variance += v.clone();
            });
        if current_variance < min_variance {
            min_variance = current_variance;
            selected_index = i;
        }
    }
    selected_index
}

pub fn split<T>(node: &mut Node<T>) -> Option<Node<T>> {
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
    pub fn test_mean_calculation() {
        let points = get_test_points();
        let mean = calculate_mean(&points, 0, points.len());
        assert_eq!(mean[0], 0.);
        assert_eq!(mean[1], 4.);
    }

    #[test]
    pub fn test_variance_calculation() {
        let points = get_test_points();
        let variance = calculate_variance(&points, 0, points.len());
        assert_eq!(variance[0], 0.);
        assert_eq!(variance[1], 14.);
    }

    #[test]
    pub fn test_range_variance_calculation() {
        let points = get_test_points();
        let variance = calculate_variance(&points, 0, 2);
        assert_eq!(variance[0], 0.);
        assert_eq!(variance[1], 0.25);
    }

    #[test]
    pub fn test_choose_split_axis() {
        let origin = vec![0., 0.];
        let mut node = Node::new_leaf(&origin, 10);
        get_test_points().iter().for_each(|point| {
            node.points_mut().push(point.to_owned());
        });

        let expected_axis = 1;
        let selected_axis = choose_split_axis(&node);
        assert_eq!(expected_axis, selected_axis);
    }

    #[test]
    pub fn test_choose_split_index() {
        let origin = vec![0., 0.];
        let mut node = Node::new_leaf(&origin, 10);
        get_test_points().iter().for_each(|point| {
            node.points_mut().push(point.to_owned());
        });

        let expected_index = 3;
        let selected_index = choose_split_index(&mut node, 2, 3);
        assert_eq!(expected_index, selected_index);
    }
}
