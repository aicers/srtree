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

    pub fn min_distance(&self, point: &Point<T>) -> T {
        let distance = self.center.distance(point);
        T::zero().max(distance - self.radius)
    }

    pub fn max_distance(&self, point: &Point<T>) -> T {
        let distance = self.center.distance(point);
        distance + self.radius
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_sphere_min_distance() {
        let sphere1 = Sphere::new(Point::new(vec![0., 0.], 0), 10.);
        let point1 = Point::new(vec![15., 0.], 0);
        assert_eq!(sphere1.min_distance(&point1), 5.);
    }
}
