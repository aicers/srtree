use crate::{node::Node, shape::point::Point, Params};
use num_traits::cast;
use ordered_float::Float;
use std::{
    fmt::Debug,
    ops::{AddAssign, DivAssign, MulAssign, SubAssign},
};

pub fn bulk_load<T>(points: Vec<Point<T>>, params: &Params) -> Node<T>
where
    T: Debug + Float + AddAssign + SubAssign + MulAssign + DivAssign,
{
    if points.len() <= params.max_number_of_elements {
        return Node::create_leaf(points);
    }

    let variances = calculate_variance(&points);
    let split_dim = variances
        .iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
        .map(|(i, _)| i)
        .unwrap();

    let groups = partition_points(points, split_dim, params.min_number_of_elements, params);
    let children = groups
        .into_iter()
        .map(|group| bulk_load(group, params))
        .collect();
    Node::create_parent(children)
}

pub fn partition_points<T>(
    mut points: Vec<Point<T>>,
    split_dim: usize,
    num_slices: usize,
    params: &Params,
) -> Vec<Vec<Point<T>>>
where
    T: Debug + Float + AddAssign + SubAssign + MulAssign + DivAssign,
{
    if points.len() <= params.max_number_of_elements {
        return vec![points];
    }
    let split_dim = split_dim % params.dimension;
    let partition_size =
        calculate_partition_size(points.len(), num_slices).max(params.min_number_of_elements);
    let mut remaining = points.len() % partition_size;
    let mut entries = Vec::new();
    while !points.is_empty() {
        let left = points
            .len()
            .saturating_sub(partition_size + (remaining > 0) as usize);
        remaining = remaining.saturating_sub(1);
        points.select_nth_unstable_by(left, |a, b| {
            a.coord_at(split_dim)
                .partial_cmp(&b.coord_at(split_dim))
                .unwrap()
        });
        let slice = points.split_off(left);
        entries.push(slice);
    }
    entries
}

fn calculate_dimension_variance<T>(points: &[Point<T>], dim: usize) -> T
where
    T: Float + AddAssign + SubAssign + MulAssign + DivAssign + Debug + Copy,
{
    let mut sum = T::zero();
    let mut sum_sq = T::zero();
    for point in points {
        let coord = point.coord_at(dim);
        sum += coord;
        sum_sq += coord * coord;
    }
    let n = T::from(points.len()).unwrap();
    let mean = sum / n;
    let mean_sq = mean * mean;
    let variance = sum_sq / n - mean_sq;
    variance
}

fn calculate_variance<T>(points: &[Point<T>]) -> Vec<T>
where
    T: Float + AddAssign + SubAssign + MulAssign + DivAssign + Debug + Copy,
{
    let mut variances = Vec::new();
    for dim in 0..points[0].dimension() {
        let variance = calculate_dimension_variance(points, dim);
        variances.push(variance);
    }
    variances
}

fn calculate_partition_size(n: usize, num_slices: usize) -> usize {
    let n: f64 = cast(n).unwrap();
    let num_slices: f64 = cast(num_slices).unwrap();
    let partition_size = (n / num_slices).floor();

    cast(partition_size).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_calculate_variance() {
        let points = vec![
            Point::with_coords(vec![1.0, 2.0, 3.0]),
            Point::with_coords(vec![2.0, 3.0, 4.0]),
            Point::with_coords(vec![3.0, 4.0, 5.0]),
        ];
        let variance = calculate_dimension_variance(&points, 0);
        assert!((variance - 2.0 / 3.0).abs() <= 0.00001);
    }
}
