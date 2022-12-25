use super::{line_iterator::LineIterator, vector::Vector};

pub struct Line<T> {
    pub start: T,
    pub end: T,
}
impl<T> Line<T> {
    pub fn new(start: T, end: T) -> Self {
        Self { start, end }
    }
}

pub type LineVector<const C: usize, T> = Line<Vector<C, T>>;
pub type LineVectori32<const C: usize> = LineVector<C, i32>;

impl<const C: usize> LineVectori32<C> {
    pub fn iter(&self) -> LineIterator<C> {
        LineIterator::new(self.start, self.end)
    }
}

pub type LineV2i32 = LineVectori32<2>;
