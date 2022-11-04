use num_traits::Float;
use num_traits::FromPrimitive;
use num_traits::Zero;
use std::ops::AddAssign;
use std::ops::DivAssign;
use std::ops::MulAssign;
use std::ops::SubAssign;

pub struct Euclidean<T> {
    default: T
}

impl <T> Euclidean<T>
where
    T: Float + Zero + FromPrimitive + AddAssign + SubAssign + DivAssign + MulAssign,
{
    pub fn distance(point1: &Vec<T>, point2: &Vec<T>) -> T {
        if point1.len() != point2.len() {
            return T::infinity()
        }
        let mut distance = T::zero();
        for i in 0..point1.len() {
            distance += (point1[i] - point2[i]).powi(2);
        }
        distance.sqrt()
    }
}

#[cfg(test)]
mod tests{
    use std::vec;

    use super::*;

    #[test]
    pub fn test_distance(){
        let point1 = vec![1.,0.,0.];
        let point2 = vec![2.,0.,0.];
        assert_eq!(Euclidean::distance(&point1, &point2), 1.);
    }

    #[test]
    pub fn test_distance_different_dimensions(){
        let point1 = vec![1.,2.];
        let point2 = vec![1.,2.,3.];
        assert_eq!(Euclidean::distance(&point1, &point2), Float::infinity());
    }
}
