use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Mul;
use std::ops::MulAssign;
use std::ops::Sub;
use std::ops::SubAssign;

#[derive(Debug)]
pub struct Rect<T, const dimension: usize> {
    low: [T; dimension],
    high: [T; dimension]
}

impl <const dimension: usize, T> Rect<T, dimension> where T: Copy + Add<Output = T> + AddAssign + Sub<Output = T> + SubAssign + Mul<Output = T> + MulAssign + Ord + Default {

    pub fn new(low: [T; dimension], high: [T; dimension]) -> Rect<T, dimension>{
        Rect { low, high }
    }

    pub fn from_point(point: [T; dimension]) -> Rect<T, dimension> {
        Rect::new(point, point)
    }

    pub fn from_points(points: &Vec<[T; dimension]>) -> Rect<T, dimension> {
        // todo
        Rect::from_point([T::default(); dimension])
    }

    pub fn area(&self) -> T {
        if dimension == 0 {
            return Default::default();
        }
        let mut area = self.high[0] - self.low[0];
        for i in 1..dimension {
            area *= self.high[i] - self.low[i];
        }
        area
    }

    pub fn contains(&self, point: [T; dimension]) -> bool {
        for i in 0..dimension {
            if point[i] < self.low[i] || self.high[i] < point[i] {
                return false;
            }
        }
        true
    }

    pub fn intersects(&self, rect: Rect<T, dimension>) -> bool {
        for i in 0..dimension {
            if rect.high[i] < self.low[i] || self.high[i] < rect.low[i] {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_rect_area() {
        let rec1 = Rect::new([0,0], [10,10]);
        let expected = 100;
        assert_eq!(expected, rec1.area());
    }
    
    #[test]
    pub fn test_rect_area_from_point() {
        let rec1 = Rect::from_point([0,0]);
        let expected = 0;
        assert_eq!(expected, rec1.area());
    }

    #[test]
    pub fn test_rect_contains_point(){
        let rec = Rect::new([0,0], [10,10]);
        let point = [5,5];
        assert!(rec.contains(point));
    }
    
    #[test]
    pub fn test_rect_doesnot_contain_point(){
        let rec = Rect::new([0,0], [10,10]);
        let point = [5,15];
        assert_eq!(rec.contains(point), false)
    }
    
    #[test]
    pub fn test_rect_intersects_rect(){
        let rec1 = Rect::new([0,0], [10,10]);
        let rec2 = Rect::new([8,8],[15,15]);
        assert!(rec1.intersects(rec2));   
    }
    
    #[test]
    pub fn test_rect_doesnot_intersect_rect(){
        let rec1 = Rect::new([0,0], [10,10]);
        let rec2 = Rect::new([20,20],[30,30]);
        assert_eq!(rec1.intersects(rec2), false);
    }
    
    #[test]
    pub fn test_rect_intersects_its_clone(){
        let rec1 = Rect::new([0,0], [10,10]);
        let rec2 = Rect::new([0,0],[10,10]);
        assert!(rec1.intersects(rec2));
    }

    #[test]
    pub fn test_rect_intersects_smaller_rect(){
        let rec1 = Rect::new([0,0], [10,10]);
        let rec2 = Rect::new([2,2],[8,8]);
        assert!(rec1.intersects(rec2));
    }
}