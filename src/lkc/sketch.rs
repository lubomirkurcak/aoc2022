use std::collections::VecDeque;

pub trait Bag<T> {
    fn new() -> Self;
    fn put(&mut self, t: T);
    fn get(&mut self) -> Option<T>;
    fn len(&self) -> usize;

    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

pub struct StackBag<T>(Vec<T>);
pub struct QueueBag<T>(VecDeque<T>);

impl<T> Bag<T> for StackBag<T> {
    fn put(&mut self, t: T) {
        self.0.push(t)
    }
    fn get(&mut self) -> Option<T> {
        self.0.pop()
    }
    fn len(&self) -> usize {
        self.0.len()
    }
    fn new() -> Self {
        Self(vec![])
    }
}
impl<T> Bag<T> for QueueBag<T> {
    fn put(&mut self, t: T) {
        self.0.push_back(t)
    }
    fn get(&mut self) -> Option<T> {
        self.0.pop_front()
    }
    fn len(&self) -> usize {
        self.0.len()
    }
    fn new() -> Self {
        Self(VecDeque::new())
    }
}