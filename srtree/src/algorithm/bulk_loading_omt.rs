use crate::{node::Node, shape::point::Point, Params};
use num_traits::cast;
use ordered_float::Float;
use std::{
    fmt::Debug,
    ops::{AddAssign, DivAssign, MulAssign, SubAssign},
};

use super::split;

// Overlap Minimizing Top-Down (OMT) Bulk-loading Algorithm for R-trees
// Read more here: https://ceur-ws.org/Vol-74/files/FORUM_18.pdf

pub fn bulk_load<T>(points: Vec<Point<T>>, params: &Params) -> Node<T>
where
    T: Debug + Float + AddAssign + SubAssign + MulAssign + DivAssign,
{
    if points.len() <= params.max_number_of_elements {
        return Node::create_leaf(points);
    }
    let height = calculate_height(points.len(), params.max_number_of_elements);
    let n_subtree = calculate_n_subtree(params.max_number_of_elements, height);
    let num_slices = calculate_num_slices(points.len(), n_subtree, params.dimension);
    if num_slices <= 1 {
        return Node::create_leaf(points);
    }
    let groups = partition_points(points, 0, num_slices, params);
    let children = groups
        .into_iter()
        .map(|group| bulk_load(group, params))
        .collect();
    let children = partition_groups(children, 0, num_slices, params);
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
    let partition_size = calculate_partition_size(points.len(), num_slices);

    // Partition the points along this dimension into groups
    // and recursively partition each of the groups along the next dimension
    let mut entries = Vec::new();
    while !points.is_empty() {
        let mut remaining = points.len().saturating_sub(partition_size);
        points.select_nth_unstable_by(remaining, |a, b| {
            a.coord_at(split_dim)
                .partial_cmp(&b.coord_at(split_dim))
                .unwrap()
        });
        let slice = points.split_off(remaining);
        entries.extend(partition_points(slice, split_dim + 1, num_slices, params));
    }
    entries
}

pub fn partition_groups<T>(
    mut groups: Vec<Node<T>>,
    split_dim: usize,
    num_slices: usize,
    params: &Params,
) -> Vec<Node<T>>
where
    T: Debug + Float + AddAssign + SubAssign + MulAssign + DivAssign,
{
    if groups.len() <= params.max_number_of_elements {
        return groups;
    }
    let split_dim = split_dim % params.dimension;
    let partition_size = calculate_partition_size(groups.len(), num_slices);
    let mut entries = Vec::new();
    while !groups.is_empty() {
        let mut remaining = groups.len().saturating_sub(partition_size);
        groups.select_nth_unstable_by(remaining, |a, b| {
            a.get_sphere().center.coord_at(split_dim)
                .partial_cmp(&b.get_sphere().center.coord_at(split_dim))
                .unwrap()
        });
        let slice = groups.split_off(remaining);
        let partitions = partition_groups(slice, split_dim + 1, num_slices, params);
        if partitions.len() == 1 {
            entries.extend(partitions);
        } else {
            entries.push(Node::create_parent(partitions));
        }
    }
    entries
}

// OMT Eq. 1
pub fn calculate_height(n: usize, m: usize) -> usize {
    let n: f64 = cast(n).unwrap();
    let m: f64 = cast(m).unwrap();

    let height = n.log(m).ceil();
    cast(height).unwrap_or(1).max(1) // the height must be at least 1
}

// OMT Eq. 2
pub fn calculate_n_subtree(m: usize, height: usize) -> usize {
    let m: f64 = cast(m).unwrap();
    let height: f64 = cast(height).unwrap();

    let n_subtree = m.powf(height - 1.);
    cast(n_subtree).unwrap_or(1)
}

// OMT Eq. 3
pub fn calculate_num_slices(n: usize, n_subtree: usize, dim: usize) -> usize {
    let n: f64 = cast(n).unwrap();
    let n_subtree: f64 = cast(n_subtree).unwrap();
    let dim: f64 = cast(dim).unwrap();

    let s = (n / n_subtree).powf(1. / dim).floor();
    cast(s).unwrap_or(2).max(2) // the number of slices must be at least 2
}

pub fn calculate_partition_size(n: usize, num_slices: usize) -> usize {
    let n: f64 = cast(n).unwrap();
    let num_slices: f64 = cast(num_slices).unwrap();
    let partition_size = (n / num_slices).ceil();

    cast(partition_size).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_calculate_height() {
        let n = 101;
        let m = 10;
        let height = calculate_height(n, m);
        assert_eq!(height, 3);
    }

    #[test]
    pub fn test_calculate_n_subtree() {
        let m = 10;
        let height = 2;
        let n_subtree = calculate_n_subtree(m, height);
        assert_eq!(n_subtree, 10);
    }

    #[test]
    pub fn test_calculate_num_slices() {
        let n = 101;
        let n_subtree = 100;
        let dim = 2;
        let num_slices = calculate_num_slices(n, n_subtree, dim);
        assert_eq!(num_slices, 2);
    }
}
