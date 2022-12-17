pub mod aabb;
pub mod array2d;
pub mod geometric_traits;
pub mod linear_index;
pub mod math;
pub mod transformations;
pub mod v2;

#[cfg(test)]
mod tests {
    use super::v2::V2;

    #[test]
    fn v2_eq() {
        let a = V2::new(0, 0);
        let b = V2::new(0, 0);
        assert!(a == b);
        assert_eq!(a, b);
    }
}
