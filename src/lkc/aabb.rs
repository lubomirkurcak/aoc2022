use std::ops::Sub;

use super::{geometric_traits::CoverObject, vector::Vector};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Aabb<const C: usize, T> {
    pub min: Vector<C, T>,
    pub max: Vector<C, T>,
}

impl<const C: usize, T: std::cmp::Ord + Copy> CoverObject<Vector<C, T>> for Aabb<C, T> {
    fn cover(&mut self, point: &Vector<C, T>) {
        #[allow(clippy::needless_range_loop)]
        for x in 0..C {
            self.min.values[x] = std::cmp::min(self.min.values[x], point.values[x]);
            self.max.values[x] = std::cmp::max(self.max.values[x], point.values[x]);
        }
    }
}

impl<const C: usize, T> Aabb<C, T>
where
    T: std::cmp::Ord + Copy,
    Vector<C, T>: Sub<Output = Vector<C, T>>,
{
    pub fn new(min: Vector<C, T>, max: Vector<C, T>) -> Self {
        Self { min, max }
    }

    #[allow(dead_code)]
    pub fn covering(points: &[Vector<C, T>]) -> Option<Self> {
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
    pub fn dim(&self) -> Vector<C, T> {
        self.max - self.min
    }
}

pub type Aabb2<T> = Aabb<2, T>;
pub type Aabb3<T> = Aabb<3, T>;
pub type Aabb4<T> = Aabb<4, T>;

#[cfg(test)]
mod tests {
    use crate::lkc::{aabb::Aabb2, vector::V2};

    #[test]
    fn aabb_covering() {
        let points = vec![V2::from_xy(2, 0), V2::from_xy(0, 2)];
        let aabb = Aabb2::covering(&points);
        assert_eq!(
            aabb.unwrap(),
            Aabb2::new(V2::from_xy(0, 0), V2::from_xy(2, 2))
        );
    }
}
