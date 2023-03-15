use crate::{node::Node, shape::point::Point, Params};
use num_traits::cast;
use ordered_float::Float;
use std::{
    fmt::Debug,
    ops::{AddAssign, DivAssign, MulAssign, SubAssign},
};

// Overlap Minimizing Top-Down (OMT) Bulk-loading Algorithm for R-trees
// Read more here: https://ceur-ws.org/Vol-74/files/FORUM_18.pdf

pub fn bulk_load<T>(points: Vec<Point<T>>, params: &Params) -> Node<T>
where
    T: Debug + Float + AddAssign + SubAssign + MulAssign + DivAssign,
{
    if points.len() <= params.max_number_of_elements {
        return Node::create_leaf(points);
    }
    let num_slices = calculate_num_slices(
        points.len(),
        params.max_number_of_elements,
        params.dimension,
    );
    let groups = partition_points(points, 0, num_slices, params);
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
    if split_dim == params.dimension || points.len() <= params.max_number_of_elements {
        return vec![points];
    }
    let partition_size = calculate_partition_size(points.len(), num_slices);

    // Partition the points along this dimension into groups
    // and recursively partition each of the groups along the next dimension
    let mut entries = Vec::new();
    while !points.is_empty() {
        let remaining = points.len().saturating_sub(partition_size);
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

pub fn calculate_num_slices(n: usize, m: usize, dim: usize) -> usize {
    let n: f64 = cast(n).unwrap();
    let m: f64 = cast(m).unwrap();
    let dim: f64 = cast(dim).unwrap();

    let height = n.log(m).ceil(); // OMT Eq. 1
    let n_subtree = m.powf(height - 1.); // OMT Eq. 2
    let s = (n / n_subtree).powf(1. / dim).round(); // OMT Eq. 3: using round() instead of floor() to get more slices if possible

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
    use crate::SRTree;
    use rand::{rngs::StdRng, Rng, SeedableRng};

    #[test]
    pub fn test_bulk_load() {
        let mut pts = Vec::new();
        let mut rng = StdRng::from_seed(*b"PiH6Xi3GBBXhTK6UsXJYngHaF3fx4aYS");
        for _ in 0..100 {
            let mut point = [0.; 2];
            for item in point.iter_mut().take(2) {
                *item = (rng.gen::<f64>() * 100.).floor();
            }
            pts.push(point);
        }
        let params = Params::new(2, 5, 2, true).unwrap();
        let pts: Vec<Vec<f64>> = pts.iter().map(|p| p.to_vec()).collect();
        let tree = SRTree::bulk_load(&pts, params);
        assert_eq!(tree.total_childen(), pts.len());
    }
}
