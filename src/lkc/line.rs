use super::{line_iterator::LineIterator, v2::V2i32};

pub struct Line<T> {
    pub start: T,
    pub end: T,
}

impl<T> Line<T> {
    pub fn new(start: T, end: T) -> Self {
        Self { start, end }
    }
}

impl LineV2i32 {
    pub fn iter(&self) -> LineIterator {
        LineIterator::new(self.start, self.end)
    }
}

pub type LineV2i32 = Line<V2i32>;
