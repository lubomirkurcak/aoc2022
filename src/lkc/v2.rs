use std::{
    ops::{Add, Mul, Sub},
    str::FromStr,
};

#[derive(Hash, Debug, Clone, Copy, PartialEq, Eq)]
pub struct V2<T> {
    pub x: T,
    pub y: T,
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

impl<T: Add<Output = T>> Add for V2<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
// mismatched types
//       expected struct `lkc::v2::V2<T>`
// found associated type `<lkc::v2::V2<T> as std::ops::Sub>::Output`
// consider constraining the associated type `<lkc::v2::V2<T> as std::ops::Sub>::Output` to `lkc::v2::V2<T>`
impl<T: Sub<Output = T>> Sub for V2<T> {
    type Output = V2<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
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

// pub type V2i32 = V2<i32>;
pub type V2usize = V2<usize>;
