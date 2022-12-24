pub trait LinearIndex<I> {
    fn index(&self, i: I) -> Option<usize>;
    fn unindex(&self, i: usize) -> Option<I>;
    fn is_in_bounds(&self, i: I) -> bool;
}
