use std::{
    fmt::Display,
    ops::{Add, Mul, Sub},
};

use super::vector::V3;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Multivector3<T> {
    pub s: T,
    pub x: T,
    pub y: T,
    pub z: T,
    pub xy: T,
    pub yz: T,
    pub xz: T,
    pub xyz: T,
}

impl<T: Default> Multivector3<T> {
    pub fn scalar(s: T) -> Self {
        Self {
            s,
            ..Default::default()
        }
    }

    pub fn x_axis(x: T) -> Self {
        Self {
            x,
            ..Default::default()
        }
    }

    pub fn y_axis(y: T) -> Self {
        Self {
            y,
            ..Default::default()
        }
    }

    pub fn z_axis(z: T) -> Self {
        Self {
            z,
            ..Default::default()
        }
    }
}

impl<T: Default + Copy> Multivector3<T> {
    pub fn from_vector(v: V3<T>) -> Self {
        Self {
            x: v.x(),
            y: v.y(),
            z: v.z(),
            ..Default::default()
        }
    }
}

impl<T: Add<Output = T>> Add for Multivector3<T> {
    type Output = Self;

    fn add(self, b: Self) -> Self::Output {
        Self {
            s: self.s + b.s,
            x: self.x + b.x,
            y: self.y + b.y,
            z: self.z + b.z,
            xy: self.xy + b.xy,
            yz: self.yz + b.yz,
            xz: self.xz + b.xz,
            xyz: self.xyz + b.xyz,
        }
    }
}

impl<T: Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T>> Mul for Multivector3<T> {
    type Output = Self;

    fn mul(self, b: Self) -> Self::Output {
        Self {
            s: self.s * b.s + self.x * b.x + self.y * b.y - self.xy * b.xy + self.z * b.z
                - self.xz * b.xz
                - self.yz * b.yz
                - self.xyz * b.xyz,
            x: self.s * b.x + self.x * b.s - self.y * b.xy + self.xy * b.y - self.z * b.xz
                + self.xz * b.z
                - self.yz * b.xyz
                - self.xyz * b.yz,
            y: self.s * b.y + self.x * b.xy + self.y * b.s - self.xy * b.x - self.z * b.yz
                + self.xz * b.xyz
                + self.yz * b.z
                + self.xyz * b.xz,
            xy: self.s * b.xy + self.x * b.y - self.y * b.x + self.xy * b.s + self.z * b.xyz
                - self.xz * b.yz
                + self.yz * b.xz
                + self.xyz * b.z,
            z: self.s * b.z + self.x * b.xz + self.y * b.yz - self.xy * b.xyz + self.z * b.s
                - self.xz * b.x
                - self.yz * b.y
                - self.xyz * b.xy,
            xz: self.s * b.xz + self.x * b.z - self.y * b.xyz + self.xy * b.yz - self.z * b.x
                + self.xz * b.s
                - self.yz * b.xy
                - self.xyz * b.y,
            yz: self.s * b.yz + self.x * b.xyz + self.y * b.z - self.xy * b.xz - self.z * b.y
                + self.xz * b.xy
                + self.yz * b.s
                + self.xyz * b.x,
            xyz: self.s * b.xyz + self.x * b.yz - self.y * b.xz + self.xy * b.z + self.z * b.xy
                - self.xz * b.y
                + self.yz * b.x
                + self.xyz * b.s,
        }
    }
}

impl<T: Copy + Default + Add<Output = T> + Sub<Output = T> + Mul<Output = T>> Mul<V3<T>>
    for Multivector3<T>
{
    type Output = Self;

    fn mul(self, rhs: V3<T>) -> Self::Output {
        self * Self::from_vector(rhs)
    }
}

impl Display for Multivector3<f32> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let a = [
            self.s, self.x, self.y, self.z, self.xy, self.yz, self.xz, self.xyz,
        ];
        let b = ["", "x", "y", "z", "xy", "yz", "xz", "xyz"];

        let mut acc = 0;

        for i in 0..8 {
            let x = a[i];
            let name = b[i];
            if x != 0.0 {
                if acc == 0 {
                    write!(f, "({:4.1}{}", x, name)?;
                } else {
                    write!(f, " {:+4.1}{}", x, name)?;
                }
                acc += 1;
            }
        }

        if acc == 0 {
            write!(f, "(0)")
        } else {
            write!(f, ")")
        }
    }
}

impl Display for Multivector3<i32> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let a = [
            self.s, self.x, self.y, self.z, self.xy, self.yz, self.xz, self.xyz,
        ];
        let b = ["", "x", "y", "z", "xy", "yz", "xz", "xyz"];

        let mut acc = 0;

        for i in 0..8 {
            let x = a[i];
            let name = b[i];
            if x != 0 {
                if acc == 0 {
                    write!(f, "({}{}", x, name)?;
                } else {
                    write!(f, " {:+}{}", x, name)?;
                }
                acc += 1;
            }
        }

        if acc == 0 {
            write!(f, "(0)")
        } else {
            write!(f, ")")
        }
    }
}
