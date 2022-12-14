use ordered_float::Float;
use std::{
    fmt::Debug,
    ops::{AddAssign, DivAssign, MulAssign, SubAssign},
};

pub fn euclidean<T>(point1: &[T], point2: &[T]) -> T
where
    T: Debug + Float + AddAssign + SubAssign + MulAssign + DivAssign,
{
    euclidean_squared(point1, point2).sqrt()
}

pub fn euclidean_squared<T>(point1: &[T], point2: &[T]) -> T
where
    T: Debug + Float + AddAssign + SubAssign + MulAssign + DivAssign,
{
    if point1.len() != point2.len() {
        return T::infinity();
    }
    let mut distance = T::zero();
    for i in 0..point1.len() {
        distance += (point1[i] - point2[i]).powi(2);
    }
    distance
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::vec;

    #[test]
    pub fn test_distance() {
        let point1 = vec![1., 0., 0.];
        let point2 = vec![2., 0., 0.];
        assert_eq!(euclidean(&point1, &point2), 1.);
    }

    #[test]
    pub fn test_distance_different_dimensions() {
        let point1 = vec![1., 2.];
        let point2 = vec![1., 2., 3.];
        assert_eq!(euclidean(&point1, &point2), Float::infinity());
    }
}
