use ordered_float::Float;
use std::ops::{AddAssign, DivAssign, MulAssign, SubAssign};

#[allow(dead_code)]
pub fn euclidean<T>(point1: &Vec<T>, point2: &Vec<T>) -> T
where
    T: Float + AddAssign + SubAssign + MulAssign + DivAssign,
{
    if point1.len() != point2.len() {
        return T::infinity();
    }
    let mut distance = T::zero();
    for i in 0..point1.len() {
        distance += (point1[i] - point2[i]).powi(2);
    }
    distance.sqrt()
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
