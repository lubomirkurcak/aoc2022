pub trait Gcd {
    fn gcd(a: Self, b: Self) -> Self;
    fn lcm(a: Self, b: Self) -> Self;
}

// NOTE(lubo): adapted https://rosettacode.org/wiki/Least_common_multiple#Rust
macro_rules! gcd {
    ($($t:ty),*) => {
        $(
        impl Gcd for $t {
            fn gcd(a: Self, b: Self) -> Self {
                match ((a, b), (a & 1, b & 1)) {
                    ((x, y), _) if x == y => y,
                    ((0, x), _) | ((x, 0), _) => x,
                    ((x, y), (0, 1)) | ((y, x), (1, 0)) => Self::gcd(x >> 1, y),
                    ((x, y), (0, 0)) => Self::gcd(x >> 1, y >> 1) << 1,
                    ((x, y), (1, 1)) => {
                        let (x, y) = (std::cmp::min(x, y), std::cmp::max(x, y));
                        Self::gcd((y - x) >> 1, x)
                    }
                    _ => unreachable!(),
                }
            }
            fn lcm(a: Self, b: Self) -> Self {
                a * (b / Self::gcd(a, b))
            }
        })*
    };
}

gcd!(usize, i32, i64);

pub trait AbsoluteValue
where
    Self: Sized,
{
    fn abs(&self) -> Option<Self>;
}

macro_rules! checked_absolute_value {
    ($($t:ty),*) => {
        $(
        impl AbsoluteValue for $t {
    fn abs(&self) -> Option<Self> {
        self.checked_abs()
    }
        })*
    };
}

checked_absolute_value!(i32);

macro_rules! identity_absolute_value {
    ($($t:ty),*) => {
        $(
impl AbsoluteValue for $t {
    fn abs(&self) -> Option<Self> {
        Some(*self)
    }
        })*
    };
}

identity_absolute_value!(usize);

pub trait InclusiveMin<T> {
    fn inclusive_min(&self) -> &T;
}
pub trait InclusiveMax<T> {
    fn inclusive_max(&self) -> &T;
}
pub trait ExclusiveMax<T> {
    fn exclusive_max(&self) -> &T;
}

pub trait Interval
where
    Self: Sized,
{
    fn interval_intersection(&self, other: &Self) -> Option<Self>;
    fn interval_union(&self, other: &Self) -> Option<Self>;
}

// impl<T> InclusiveIntervalOverlap for T
// where
//     T: PartialOrd,
//     Self: InclusiveMin<T> + InclusiveMax<T>,
// {
//     fn inclusive_interval_overlap_test(&self, other: &Self) -> bool {
//         // a1 >= b0 && a0 <= b1
//         self.inclusive_max() >= other.inclusive_min()
//             && self.inclusive_min() <= other.inclusive_max()
//     }
//
//     fn inclusive_interval_union(&self, other: &Self) -> Self {
//         todo!()
//     }
// }

impl<T> InclusiveMin<T> for std::ops::Range<T> {
    fn inclusive_min(&self) -> &T {
        &self.start
    }
}
impl<T> ExclusiveMax<T> for std::ops::Range<T> {
    fn exclusive_max(&self) -> &T {
        &self.end
    }
}
impl Interval for std::ops::Range<i32> {
    fn interval_intersection(&self, other: &Self) -> Option<Self> {
        let mut a0 = *self.inclusive_min();
        let mut a1 = *self.exclusive_max();
        let mut b0 = *other.inclusive_min();
        let mut b1 = *other.exclusive_max();

        if a0 > b0 {
            std::mem::swap(&mut a0, &mut b0);
            std::mem::swap(&mut a1, &mut b1);
        }

        // NOTE(lubo): Current policy is to return None
        // a1 < b0 to get Some(a..a)
        // a1 <= b0 to get None
        if a1 <= b0 {
            None
        } else {
            Some(b0..std::cmp::min(a1, b1))
        }
    }

    fn interval_union(&self, other: &Self) -> Option<Self> {
        let mut a0 = *self.inclusive_min();
        let mut a1 = *self.exclusive_max();
        let mut b0 = *other.inclusive_min();
        let mut b1 = *other.exclusive_max();

        if a0 > b0 {
            std::mem::swap(&mut a0, &mut b0);
            std::mem::swap(&mut a1, &mut b1);
        }

        if a1 < b0 {
            None
        } else {
            Some(a0..std::cmp::max(a1, b1))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::lkc::math::Interval;

    #[test]
    fn interval_overlap_abab() {
        let a = 0..2;
        let b = 1..3;
        assert_eq!(a.interval_union(&b), Some(0..3));
        assert_eq!(b.interval_union(&a), Some(0..3));
        assert_eq!(a.interval_intersection(&b), Some(1..2));
        assert_eq!(b.interval_intersection(&a), Some(1..2));
    }
    #[test]
    fn interval_overlap_abba() {
        let a = 0..3;
        let b = 1..2;
        assert_eq!(a.interval_union(&b), Some(0..3));
        assert_eq!(b.interval_union(&a), Some(0..3));
        assert_eq!(a.interval_intersection(&b), Some(1..2));
        assert_eq!(b.interval_intersection(&a), Some(1..2));
    }
    #[test]
    fn interval_overlap_aabb() {
        let a = 0..1;
        let b = 2..3;
        assert_eq!(a.interval_union(&b), None);
        assert_eq!(b.interval_union(&a), None);
        assert_eq!(a.interval_intersection(&b), None);
        assert_eq!(b.interval_intersection(&a), None);
    }
    #[test]
    fn interval_overlap_abx() {
        let a = 0..2;
        let b = 1..2;
        assert_eq!(a.interval_union(&b), Some(0..2));
        assert_eq!(b.interval_union(&a), Some(0..2));
        assert_eq!(a.interval_intersection(&b), Some(1..2));
        assert_eq!(b.interval_intersection(&a), Some(1..2));
    }
    #[test]
    fn interval_overlap_xab() {
        let a = 0..3;
        let b = 0..2;
        assert_eq!(a.interval_union(&b), Some(0..3));
        assert_eq!(b.interval_union(&a), Some(0..3));
        assert_eq!(a.interval_intersection(&b), Some(0..2));
        assert_eq!(b.interval_intersection(&a), Some(0..2));
    }
    #[test]
    fn interval_overlap_axb() {
        let a = 0..2;
        let b = 2..3;
        assert_eq!(a.interval_union(&b), Some(0..3));
        assert_eq!(b.interval_union(&a), Some(0..3));

        assert_eq!(a.interval_intersection(&b), None);
        assert_eq!(b.interval_intersection(&a), None);
        // assert_eq!(a.interval_intersection(&b), Some(2..2));
        // assert_eq!(b.interval_intersection(&a), Some(2..2));
        // assert!(a.interval_intersection(&b).is_none() || a.interval_intersection(&b) == Some(2..2));
        // assert!(b.interval_intersection(&a).is_none() || b.interval_intersection(&a) == Some(2..2));
    }
}
