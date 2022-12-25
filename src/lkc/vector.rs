use std::{
    fmt::{Debug, Display},
    ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign},
    str::FromStr,
};

use super::{
    geometric_traits::{
        EuclideanDistanceSquared, IterateNeighbours, ManhattanDistance, Movement4Directions,
    },
    math::AbsoluteValue,
};

#[derive(Hash, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Vector<const C: usize, T> {
    pub values: [T; C],
}

impl<const C: usize, T: Display> Display for Vector<C, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Vector(")?;
        for x in 0..C {
            write!(f, "{}, ", self.values[x])?;
        }
        write!(f, ")")
    }
}

impl<const C: usize, T> Vector<C, T> {
    pub fn new(values: [T; C]) -> Self {
        Self { values }
    }
}

impl<const C: usize, T> Vector<C, T>
where
    T: Copy,
    T: Add<Output = T>,
    T: Mul<Output = T>,
{
    pub fn inner(&self, rhs: Self) -> T {
        let mut result = self.values[0] * rhs.values[0];
        for x in 1..C {
            result = result + self.values[x] * rhs.values[x];
        }
        result
    }
}

impl<const C: usize, T: Add<Output = T> + Copy> Add for Vector<C, T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut values = self.values;

        #[allow(clippy::needless_range_loop)]
        for x in 0..C {
            values[x] = values[x] + rhs.values[x];
        }

        Self::Output::new(values)
    }
}
impl<const C: usize, T: AddAssign + Copy> AddAssign for Vector<C, T> {
    fn add_assign(&mut self, rhs: Self) {
        for x in 0..C {
            self.values[x] += rhs.values[x];
        }
    }
}
impl<const C: usize, T: Sub<Output = T> + Copy> Sub for Vector<C, T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut values = self.values;

        #[allow(clippy::needless_range_loop)]
        for x in 0..C {
            values[x] = values[x] - rhs.values[x];
        }

        Self::Output::new(values)
    }
}
impl<const C: usize, T: SubAssign + Copy> SubAssign for Vector<C, T> {
    fn sub_assign(&mut self, rhs: Self) {
        for x in 0..C {
            self.values[x] -= rhs.values[x];
        }
    }
}

impl<const C: usize, T> Mul<T> for Vector<C, T>
where
    T: Mul<Output = T> + Copy,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        let mut values = self.values;

        #[allow(clippy::needless_range_loop)]
        for x in 0..C {
            values[x] = values[x] * rhs;
        }

        Self::Output::new(values)
    }
}
impl<const C: usize, T: MulAssign + Copy> MulAssign<T> for Vector<C, T> {
    fn mul_assign(&mut self, rhs: T) {
        for x in 0..C {
            self.values[x] *= rhs;
        }
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

impl<const C: usize, T> Mul<Vector<C, T>> for Scalar<T>
where
    T: Copy,
    T: Mul<T, Output = T>,
{
    type Output = Vector<C, T>;

    fn mul(self, rhs: Vector<C, T>) -> Self::Output {
        rhs * self.value
    }
}

impl<const C: usize, T: FromStr + Debug> FromStr for Vector<C, T> {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let vec_values = s
            .split(',')
            .map(|x| x.trim().parse::<T>())
            .collect::<Result<Vec<_>, _>>();

        if let Ok(vec_values) = vec_values {
            let values: [T; C] = vec_values.try_into().unwrap();
            Ok(Self::new(values))
        } else {
            Err(())
        }
    }
}

impl<const C: usize, T> ManhattanDistance<Vector<C, T>, T> for Vector<C, T>
where
    T: Copy,
    T: Add<Output = T>,
    T: Sub<Output = T>,
    T: AbsoluteValue,
{
    fn manhattan_distance(&self, other: &Self) -> T {
        let delta = *other - *self;
        let mut result = delta.values[0].abs().unwrap();
        for i in 0..C {
            result = result + delta.values[i].abs().unwrap();
        }
        result
    }
}

impl<const C: usize, T> EuclideanDistanceSquared<Vector<C, T>, T> for Vector<C, T>
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

macro_rules! movement4directions {
    ($($t:ty),*) => {
        $(
            impl<const C: usize> IterateNeighbours<()> for Vector<C, $t> {
                fn neighbours(&self, _context: &()) -> Vec<Self> {
                    let mut results = vec![];

                    for i in 0..C {
                        if let Some(a) = self.values[i].checked_add(1) {
                            let mut b = self.clone();
                            b.values[i] = a;
                            results.push(b);
                        }
                        if let Some(a) = self.values[i].checked_sub(1) {
                            let mut b = self.clone();
                            b.values[i] = a;
                            results.push(b);
                        }
                    }

                    results
                }
            }
        )*
    };
}

movement4directions!(i32, usize);

pub type V2<T> = Vector<2, T>;
pub type V3<T> = Vector<3, T>;
pub type V4<T> = Vector<4, T>;

pub type V2i32 = V2<i32>;
pub type V2usize = V2<usize>;
pub type V2f32 = V2<f32>;
pub type V3i32 = V3<i32>;
pub type V3usize = V3<usize>;
pub type V3f32 = V3<f32>;
pub type V4i32 = V4<i32>;
pub type V4usize = V4<usize>;
pub type V4f32 = V4<f32>;

impl<T: Copy> V2<T> {
    pub fn from_xy(x: T, y: T) -> Self {
        Self { values: [x, y] }
    }
    pub fn x(&self) -> T {
        self.values[0]
    }
    pub fn y(&self) -> T {
        self.values[1]
    }
}

macro_rules! movement4directions {
    ($v:ident; $($t:ty),*) => {
        $(
        impl Movement4Directions for $v<$t> {
            fn step_right(&self) -> Option<Self> {
                if let Some(x) = self.x().checked_add(1) {
                    Some(V2::from_xy(x, self.y()))
                } else {
                    None
                }
            }
            fn step_up(&self) -> Option<Self> {
                if let Some(y) = self.y().checked_add(1) {
                    Some(V2::from_xy(self.x(), y))
                } else {
                    None
                }
            }
            fn step_left(&self) -> Option<Self> {
                if let Some(x) = self.x().checked_sub(1) {
                    Some(V2::from_xy(x, self.y()))
                } else {
                    None
                }
            }
            fn step_down(&self) -> Option<Self> {
                if let Some(y) = self.y().checked_sub(1) {
                    Some(V2::from_xy(self.x(), y))
                } else {
                    None
                }
            }
        })*
    };
}

movement4directions!(V2; usize, i32);

impl<T: Copy> V3<T> {
    pub fn from_xyz(x: T, y: T, z: T) -> Self {
        Self { values: [x, y, z] }
    }
    pub fn x(&self) -> T {
        self.values[0]
    }
    pub fn y(&self) -> T {
        self.values[1]
    }
    pub fn z(&self) -> T {
        self.values[2]
    }
}

impl<T: Copy> V4<T> {
    pub fn from_xyzw(x: T, y: T, z: T, w: T) -> Self {
        Self {
            values: [x, y, z, w],
        }
    }
    pub fn x(&self) -> T {
        self.values[0]
    }
    pub fn y(&self) -> T {
        self.values[1]
    }
    pub fn z(&self) -> T {
        self.values[2]
    }
    pub fn w(&self) -> T {
        self.values[3]
    }
}

#[cfg(test)]
mod tests {
    use crate::lkc::vector::V3;

    #[test]
    fn v3_eq() {
        let a = V3::from_xyz(0, 0, 0);
        let b = V3::from_xyz(0, 0, 0);
        assert!(a == b);
        assert_eq!(a, b);
    }
}
