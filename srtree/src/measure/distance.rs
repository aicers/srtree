use crate::shape::point::Point;
use ordered_float::Float;

pub fn euclidean<T>(point1: &Point<T>, point2: &Point<T>) -> T
where
    T: Float + Send + Sync,
{
    euclidean_squared(point1, point2).sqrt()
}

pub fn euclidean_squared<T>(point1: &Point<T>, point2: &Point<T>) -> T
where
    T: Float + Send + Sync,
{
    let mut distance = T::zero();
    for i in 0..point1.coords.len() {
        distance = distance + (point1.coords[i] - point2.coords[i]).powi(2);
    }
    distance
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_distance() {
        let point1 = Point::new(vec![1., 0., 0.], 0);
        let point2 = Point::new(vec![2., 0., 0.], 0);
        assert_eq!(euclidean(&point1, &point2), 1.);
    }
}
