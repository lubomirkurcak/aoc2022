pub trait Gcd<T> {
    fn gcd(a: T, b: T) -> T;
    fn lcm(a: T, b: T) -> T;
}

// NOTE(lubo): adapted https://rosettacode.org/wiki/Least_common_multiple#Rust
#[macro_export]
macro_rules! gcd {
    ($($t:ty),*) => {
        $(
        impl $crate::lkc::math::Gcd<$t> for $t {
            fn gcd(a: $t, b: $t) -> $t {
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
            fn lcm(a: $t, b: $t) -> $t {
                a * (b / Self::gcd(a, b))
            }
        })*
    };
}

pub(crate) use gcd;
