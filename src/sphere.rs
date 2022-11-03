use crate::distance::Euclidean;
use num_traits::Float;
use num_traits::FromPrimitive;
use num_traits::Zero;
use std::ops::AddAssign;
use std::ops::DivAssign;
use std::ops::MulAssign;
use std::ops::SubAssign;

#[derive(Debug)]
pub struct Sphere<T> {
    pub center: Vec<T>,
    pub radius: T,
}

impl<T> Sphere<T>
where
    T: Float + Zero + FromPrimitive + AddAssign + SubAssign + DivAssign + MulAssign,
{
    pub fn new(center: Vec<T>, radius: T) -> Sphere<T> {
        Sphere { center: center, radius }
    }

    pub fn from_point(point: &Vec<T>) -> Sphere<T> {
        Sphere::new(point.clone(), T::zero())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_from_point(){
        let sphere = Sphere::from_point(&vec![1.,1.]);
        assert_eq!(sphere.radius, 0.);
    }
}
