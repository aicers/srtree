use ordered_float::Float;

#[derive(Clone)]
pub struct Point<T> {
    pub coords: Vec<T>,
    pub radius: T,
    pub index: usize,
    pub parent_index: usize,
}

impl<T> Point<T>
where
    T: Float + Send + Sync,
{
    pub fn new(coords: Vec<T>, index: usize) -> Point<T> {
        Point {
            coords,
            radius: T::zero(),
            index,
            parent_index: usize::MAX,
        }
    }

    pub fn dimension(&self) -> usize {
        self.coords.len()
    }
}
