use num_traits::Float;
use num_traits::FromPrimitive;
use num_traits::Zero;
use std::ops::AddAssign;
use std::ops::DivAssign;
use std::ops::MulAssign;
use std::ops::SubAssign;

pub struct Euclidean<T, const dimension: usize> {
    default: T
}

impl <T, const dimension: usize> Euclidean<T, dimension>
where
    T: Float + Zero + FromPrimitive + AddAssign + SubAssign + DivAssign + MulAssign,
{
    pub fn distance(point1: &[T; dimension], point2: &[T; dimension]) -> T {
        let mut distance = T::zero();
        for i in 0..dimension {
            distance += (point1[i] - point2[i]).powi(2);
        }
        distance.sqrt()
    }
}