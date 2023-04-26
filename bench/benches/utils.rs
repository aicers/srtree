use ordered_float::{Float, OrderedFloat};
use rstar::{RStarInsertionStrategy, RTree, RTreeParams};
use std::cmp::Ordering;

pub struct Neighbor<T>
where
    T: Float,
{
    pub distance: OrderedFloat<T>,
    pub point_index: usize,
}

impl<T> Neighbor<T>
where
    T: Float,
{
    pub fn new(distance: OrderedFloat<T>, point_index: usize) -> Neighbor<T> {
        Neighbor {
            distance,
            point_index,
        }
    }
}

impl<T> Ord for Neighbor<T>
where
    T: Float,
{
    #[must_use]
    fn cmp(&self, other: &Self) -> Ordering {
        self.distance.cmp(&other.distance)
    }
}

impl<T> PartialOrd for Neighbor<T>
where
    T: Float,
{
    #[must_use]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.distance.partial_cmp(&other.distance)
    }
}

impl<T> Eq for Neighbor<T> where T: Float {}

impl<T> PartialEq for Neighbor<T>
where
    T: Float,
{
    #[must_use]
    fn eq(&self, other: &Self) -> bool {
        self.distance.eq(&other.distance)
    }
}

pub struct LargeNodeParameters;
impl RTreeParams for LargeNodeParameters {
    const MIN_SIZE: usize = 12; // 40% of MAX_SIZE
    const MAX_SIZE: usize = 31;
    const REINSERTION_COUNT: usize = 9; // 30% of MAX_SIZE
    type DefaultInsertionStrategy = RStarInsertionStrategy;
}
pub type LargeNodeRTree<T> = RTree<T, LargeNodeParameters>;
