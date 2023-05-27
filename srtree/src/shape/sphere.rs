use super::point::Point;
use ordered_float::Float;

pub struct Sphere<T> {
    pub center: Point<T>,
    pub radius: T,
}

impl<T> Sphere<T>
where
    T: Float + Send + Sync,
{
    pub fn new(center: Point<T>, radius: T) -> Sphere<T> {
        Sphere { center, radius }
    }

    pub fn from_point(point: &Point<T>) -> Sphere<T> {
        Sphere::new(point.clone(), T::zero())
    }
}
