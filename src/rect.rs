use num_traits::Float;
use num_traits::FromPrimitive;
use num_traits::Zero;
use std::ops::AddAssign;
use std::ops::DivAssign;
use std::ops::MulAssign;
use std::ops::SubAssign;

#[derive(Debug)]
pub struct Rect<T> {
    low: Vec<T>,
    high: Vec<T>
}

impl<T> Rect<T>
where
    T: Float + Zero + FromPrimitive + AddAssign + SubAssign + DivAssign + MulAssign,
{
    pub fn new(low: Vec<T>, high: Vec<T>) -> Option<Rect<T>> {
        if low.len() != high.len() {
            return None;
        }
        Some(Rect { low, high })
    }

    pub fn from_point(point: &[T]) -> Option<Rect<T>> {
        Rect::new(point.to_owned(), point.to_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_new_rect(){
        let rec1 = Rect::new(vec![1.,2.,3.],vec![1.,2.]);
        let rec2 = Rect::new(vec![1.,2.],vec![1.,2.]);
        assert!(rec1.is_none() && rec2.is_some());
    }
}
