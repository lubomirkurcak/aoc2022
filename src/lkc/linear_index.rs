pub trait LinearIndex<I> {
    fn index_unchecked(&self, i: I) -> Option<usize>;
    fn unindex(&self, i: usize) -> Option<I>;
    fn is_in_bounds(&self, i: &I) -> bool;

    fn index(&self, i: I) -> Option<usize> {
        if self.is_in_bounds(&i) {
            self.index_unchecked(i)
        } else {
            None
        }
    }
}
