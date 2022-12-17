use std::ops::Sub;

use super::{geometric_traits::CoverObject, v2::V2};

#[derive(Debug, PartialEq, Eq)]
pub struct AABB2<T> {
    pub min: V2<T>,
    pub max: V2<T>,
}

impl<T: std::cmp::Ord + Copy> CoverObject<V2<T>> for AABB2<T> {
    fn cover(&mut self, point: &V2<T>) {
        self.min.x = std::cmp::min(self.min.x, point.x);
        self.min.y = std::cmp::min(self.min.y, point.y);
        self.max.x = std::cmp::max(self.max.x, point.x);
        self.max.y = std::cmp::max(self.max.y, point.y);
    }
}

impl<T> AABB2<T>
where
    T: std::cmp::Ord + Copy,
    V2<T>: Sub<Output = V2<T>>,
{
    pub fn new(min: V2<T>, max: V2<T>) -> Self {
        Self { min, max }
    }
    pub fn covering(points: &[V2<T>]) -> Option<Self> {
        let mut iter = points.iter();
        if let Some(a) = iter.next() {
            let mut result = Self::new(*a, *a);

            for b in iter {
                result.cover(b);
            }

            Some(result)
        } else {
            None
        }
    }
    pub fn dim(&self) -> V2<T> {
        self.max - self.min
    }
}

#[cfg(test)]
mod tests {
    use crate::lkc::{aabb::AABB2, v2::V2};

    #[test]
    fn aabb_covering() {
        let points = vec![V2::new(2, 0), V2::new(0, 2)];
        let aabb = AABB2::covering(&points);
        assert_eq!(aabb.unwrap(), AABB2::new(V2::new(0, 0), V2::new(2, 2)));
    }
}
