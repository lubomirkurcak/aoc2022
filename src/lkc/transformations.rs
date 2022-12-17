use std::ops::{Add, Sub};

pub trait Transform<T> {
    fn transform(&self, object: T) -> T;
    fn inverse_transform(&self, object: T) -> T;
}

#[derive(Debug)]
pub struct Translation<T> {
    pub translation: T,
}

impl<T> Translation<T> {
    pub fn new(translation: T) -> Self {
        Self { translation }
    }
}

impl<T> Transform<T> for Translation<T>
where
    T: Copy,
    T: Add<Output = T>,
    T: Sub<Output = T>,
{
    fn transform(&self, object: T) -> T {
        object + self.translation
    }

    fn inverse_transform(&self, object: T) -> T {
        object - self.translation
    }
}
