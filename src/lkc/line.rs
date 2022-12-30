use std::ops::{Add, Mul, Sub};

use super::{line_iterator::LineIterator, vector::Vector};

#[derive(Debug, Clone, Copy)]
pub struct Line<T> {
    pub start: T,
    pub end: T,
}
impl<T> Line<T> {
    pub fn new(start: T, end: T) -> Self {
        Self { start, end }
    }
}

impl<T: Copy + Add<Output = T> + Sub<Output = T>> Line<T> {
    pub fn delta(&self) -> T {
        self.end - self.start
    }
    pub fn offset(&self, offset: T) -> Self {
        Self {
            start: self.start + offset,
            end: self.end + offset,
        }
    }
    pub fn scale<S: Copy>(&self, scale: S) -> Self
    where
        T: Mul<S, Output = T>,
    {
        Self {
            start: self.start * scale,
            end: self.end * scale,
        }
    }
}

pub type LineVector<const C: usize, T> = Line<Vector<C, T>>;
pub type LineVectori32<const C: usize> = LineVector<C, i32>;

impl<const C: usize> LineVectori32<C> {
    pub fn iter<const B: bool>(&self) -> LineIterator<B, C> {
        LineIterator::new(self.start, self.end)
    }
}

pub type LineV2i32 = LineVectori32<2>;
