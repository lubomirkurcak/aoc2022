use std::fmt::Display;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Mul;
use std::ops::Sub;
use std::ops::SubAssign;

use super::vector::Vector;

pub trait ModularDecompose<T> {
    fn modular_decompose(&self, n: T) -> (T, T);
    fn mod_n(&self, n: T) -> T {
        self.modular_decompose(n).1
    }
}

pub trait ModularAdd {
    fn add_n(&self, rhs: Self, n: Self) -> Self;
}
pub trait ModularSub {
    fn sub_n(&self, rhs: Self, n: Self) -> Self;
}
pub trait ModularMul {
    fn mul_n(&self, rhs: Self, n: Self) -> Self;
}
pub trait ModularAddAssign {
    fn addassign_n(&mut self, rhs: Self, n: Self);
}
pub trait ModularSubAssign {
    fn subassign_n(&mut self, rhs: Self, n: Self);
}

macro_rules! modular_primitives {
    ($($t:ty),*) => {$(
        impl ModularDecompose<$t> for $t {
            fn modular_decompose(&self, n: $t) -> ($t, $t) {
                let mut value = *self;
                let mut count = 0;
                while value >= n {
                    value -= n;
                    count += 1;
                }
                #[allow(unused_comparisons)]
                while value < 0 {
                    value += n;
                    count -= 1;
                }
                (count, value)
            }
        }

        impl ModularAdd for $t {
            fn add_n(&self, rhs: Self, n: Self) -> Self {
                (self.mod_n(n) + rhs.mod_n(n)).mod_n(n)
            }
        }
        impl ModularSub for $t {
            fn sub_n(&self, rhs: Self, n: Self) -> Self {
                (self.mod_n(n) + (n - rhs.mod_n(n))).mod_n(n)
            }
        }
        impl ModularMul for $t {
            fn mul_n(&self, rhs: Self, n: Self) -> Self {
                (self.mod_n(n) * rhs.mod_n(n)).mod_n(n)
            }
        }
        impl ModularAddAssign for $t {
            fn addassign_n(&mut self, rhs: Self, n: Self) {
                *self = self.add_n(rhs, n)
            }
        }
        impl ModularSubAssign for $t {
            fn subassign_n(&mut self, rhs: Self, n: Self) {
                *self = self.sub_n(rhs, n)
            }
        }
    )*};
}

modular_primitives!(usize, i32);

impl<const C: usize, T: Copy + ModularAdd> ModularAdd for Vector<C, T> {
    fn add_n(&self, rhs: Self, n: Self) -> Self {
        let mut values = self.values.clone();
        for i in 0..C {
            values[i] = self.values[i].add_n(rhs.values[i], n.values[i]);
        }
        Self { values }
    }
}

impl<const C: usize, T: Copy + ModularSub> ModularSub for Vector<C, T> {
    fn sub_n(&self, rhs: Self, n: Self) -> Self {
        let mut values = self.values.clone();
        for i in 0..C {
            values[i] = self.values[i].sub_n(rhs.values[i], n.values[i]);
        }
        Self { values }
    }
}

impl<const C: usize, T: Copy + ModularAddAssign> ModularAddAssign for Vector<C, T> {
    fn addassign_n(&mut self, rhs: Self, n: Self) {
        for i in 0..C {
            self.values[i].addassign_n(rhs.values[i], n.values[i]);
        }
    }
}

impl<const C: usize, T: Copy + ModularSubAssign> ModularSubAssign for Vector<C, T> {
    fn subassign_n(&mut self, rhs: Self, n: Self) {
        for i in 0..C {
            self.values[i].subassign_n(rhs.values[i], n.values[i]);
        }
    }
}

macro_rules! modular_type {
    ($n:ident, $t:ty) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct $n<const C: $t>($t);

        impl<const C: $t> $n<C> {
            pub fn new(value: $t) -> Self {
                Self(Self(value).mod_n(C))
            }

            pub fn get(&self) -> $t {
                self.0
            }
        }

        impl<const C: $t> ModularDecompose<$t> for $n<C> {
            fn modular_decompose(&self, n: $t) -> ($t, $t) {
                self.0.modular_decompose(n)
            }
        }

        impl<const C: $t> Add for $n<C> {
            type Output = Self;
            fn add(self, rhs: Self) -> Self {
                Self(self.0.add_n(rhs.0, C))
            }
        }

        impl<const C: $t> AddAssign for $n<C> {
            fn add_assign(&mut self, rhs: Self) {
                self.0.addassign_n(rhs.0, C)
            }
        }

        impl<const C: $t> Sub for $n<C> {
            type Output = Self;
            fn sub(self, rhs: Self) -> Self {
                Self(self.0.sub_n(rhs.0, C))
            }
        }

        impl<const C: $t> SubAssign for $n<C> {
            fn sub_assign(&mut self, rhs: Self) {
                self.0.subassign_n(rhs.0, C)
            }
        }

        impl<const C: $t> Mul for $n<C> {
            type Output = Self;
            fn mul(self, rhs: Self) -> Self {
                Self(self.0.mul_n(rhs.0, C))
            }
        }

        impl<const C: $t> Display for $n<C> {
            fn fmt(
                &self,
                f: &mut std::fmt::Formatter<'_>,
            ) -> std::result::Result<(), std::fmt::Error> {
                write!(f, "{} (mod {})", self.0, C)
            }
        }
    };
}

modular_type!(Modi32, i32);
modular_type!(Modusize, usize);

#[cfg(test)]
mod tests {
    use super::{Modi32, Modusize};

    #[test]
    fn modular_test() {
        let a = Modusize::<5>::new(13);
        assert_eq!(a, Modusize(3));
    }

    #[test]
    fn modular_test_negative() {
        let a = Modi32::<5>::new(-3);
        assert_eq!(a, Modi32(2));
    }

    #[test]
    fn modular_add_test() {
        let a = Modusize::<5>(3);
        let b = Modusize(4);
        assert_eq!(a + b, Modusize(2));
    }
}
