use num_traits::Float;
use num_traits::FromPrimitive;
use num_traits::Zero;
use std::ops::AddAssign;
use std::ops::DivAssign;
use std::ops::MulAssign;
use std::ops::SubAssign;

pub struct Point<T, const dimension: usize> {
    coords: [T; dimension],
}

impl<T, const dimension: usize> Point<T, dimension>
where
    T: Float + Zero + FromPrimitive + AddAssign + SubAssign + DivAssign + MulAssign,
{
    pub fn new(coords: [T; dimension]) -> Point<T, dimension> {
        Point { coords }
    }

    pub fn center_of(points: &Vec<[T; dimension]>) -> Point<T, dimension> {
        let mut center = [T::zero(); dimension];
        let mut count = T::zero();
        points.iter().for_each(|point| {
            for i in 0..dimension {
                center[i] += point[i];
            }
            count += T::one();
        });
        for i in 0..dimension {
            center[i] /= count;
        }
        Point::new(center)
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    pub fn test_center_calculation_from_points() {
        let mut points = Vec::new();
        points.push([5., 5.]);
        points.push([5., 10.]);
        points.push([10., 10.]);
        points.push([5., 15.]);
        let point = Point::center_of(&points);
        assert_eq!(point.coords[0], 6.25);
        assert_eq!(point.coords[1], 10.);
    }
}
