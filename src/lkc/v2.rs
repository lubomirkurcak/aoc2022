use std::{
    fmt::Display,
    ops::{Add, AddAssign, Mul, Sub, SubAssign},
    str::FromStr,
};

use super::{
    geometric_traits::{EuclideanDistanceSquared, ManhattanDistance, Movement4Directions},
    math::AbsoluteValue,
};

#[derive(Default, Hash, Debug, Clone, Copy, PartialEq, Eq)]
pub struct V2<T> {
    pub x: T,
    pub y: T,
}

impl<T: Display> Display for V2<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "V2({}, {})", self.x, self.y)
    }
}

#[derive(Debug)]
pub enum V2ConstructionError {
    Parse,
    IncorrectNumberOfValues(usize),
}

impl<T: std::str::FromStr> From<T> for V2ConstructionError {
    fn from(_: T) -> Self {
        V2ConstructionError::Parse
    }
}

impl<T> V2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T> V2<T>
where
    T: Copy,
    T: Add<Output = T>,
    T: Mul<Output = T>,
{
    pub fn inner(&self, rhs: Self) -> T {
        self.x * rhs.x + self.y * rhs.y
    }
}

impl<T: Add<Output = T>> Add for V2<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
impl<T: AddAssign> AddAssign for V2<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}
impl<T: Sub<Output = T>> Sub for V2<T> {
    type Output = V2<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
impl<T: SubAssign> SubAssign for V2<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<T> Mul<T> for V2<T>
where
    T: Mul<Output = T> + Copy,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs)
    }
}

pub struct Scalar<T> {
    pub value: T,
}

impl<T> Scalar<T> {
    pub fn new(value: T) -> Self {
        Self { value }
    }
}

impl<T> Mul<V2<T>> for Scalar<T>
where
    T: Copy,
    T: Mul<T, Output = T>,
{
    type Output = V2<T>;

    fn mul(self, rhs: V2<T>) -> Self::Output {
        V2::new(self.value * rhs.x, self.value * rhs.y)
    }
}

// // NOTE(lubo): So sad this can't be implemented!
// impl<T: Mul<T, Output = T>> Mul<V2<T>> for T {
//     type Output = V2<T>;
//
//     fn mul(self, rhs: V2<T>) -> Self::Output {
//         V2::new(self * rhs.x, self * rhs.y)
//     }
// }

impl<T: FromStr> FromStr for V2<T> {
    type Err = V2ConstructionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut values = s.split(',').map(|x| x.trim().parse::<T>());

        if let Some(x) = values.next() {
            if let Ok(x) = x {
                if let Some(y) = values.next() {
                    if let Ok(y) = y {
                        let y = y;
                        let remaining = values.count();
                        if remaining > 0 {
                            return Err(V2ConstructionError::IncorrectNumberOfValues(
                                2 + remaining,
                            ));
                        }

                        return Ok(V2::new(x, y));
                    } else {
                        return Err(V2ConstructionError::Parse);
                    }
                } else {
                    return Err(V2ConstructionError::IncorrectNumberOfValues(1));
                }
            } else {
                return Err(V2ConstructionError::Parse);
            }
        }

        Err(V2ConstructionError::IncorrectNumberOfValues(0))
    }
}

macro_rules! movement4directions {
    ($($t:ty),*) => {
        $(
        impl Movement4Directions for V2<$t> {
            fn step_right(&self) -> Option<Self> {
                if let Some(x) = self.x.checked_add(1) {
                    Some(V2::new(x, self.y))
                } else {
                    None
                }
            }
            fn step_up(&self) -> Option<Self> {
                if let Some(y) = self.y.checked_add(1) {
                    Some(V2::new(self.x, y))
                } else {
                    None
                }
            }
            fn step_left(&self) -> Option<Self> {
                if let Some(x) = self.x.checked_sub(1) {
                    Some(V2::new(x, self.y))
                } else {
                    None
                }
            }
            fn step_down(&self) -> Option<Self> {
                if let Some(y) = self.y.checked_sub(1) {
                    Some(V2::new(self.x, y))
                } else {
                    None
                }
            }
        })*
    };
}

movement4directions!(usize, i32);

impl<T> ManhattanDistance<V2<T>, T> for V2<T>
where
    T: Copy,
    T: Add<Output = T>,
    T: Sub<Output = T>,
    T: AbsoluteValue,
{
    fn manhattan_distance(&self, other: &Self) -> T {
        (other.x - self.x).abs().unwrap() + (other.y - self.y).abs().unwrap()
    }
}

impl<T> EuclideanDistanceSquared<V2<T>, T> for V2<T>
where
    T: Copy,
    T: Add<Output = T>,
    T: Sub<Output = T>,
    T: Mul<Output = T>,
{
    fn euclidean_distance_squared(&self, other: &Self) -> T {
        let delta = *other - *self;
        delta.inner(delta)
    }
}

pub type V2i32 = V2<i32>;
pub type V2usize = V2<usize>;

#[cfg(test)]
mod tests {
    use crate::lkc::v2::V2;

    #[test]
    fn v2_eq() {
        let a = V2::new(0, 0);
        let b = V2::new(0, 0);
        assert!(a == b);
        assert_eq!(a, b);
    }
}
