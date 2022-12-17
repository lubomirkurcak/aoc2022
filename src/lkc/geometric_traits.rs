use super::v2::V2usize;

pub trait CoverObject<T> {
    fn cover(&mut self, object: &T);
}

pub trait IterateNeighbours
where
    Self: std::marker::Sized,
{
    // fn neighbours(&self) -> dyn Iterator<Item = Self>;
    fn neighbours(&self) -> Vec<Self>;
}

pub trait Movement4Directions
where
    Self: std::marker::Sized,
{
    fn right(&self) -> Option<Self>;
    fn up(&self) -> Option<Self>;
    fn left(&self) -> Option<Self>;
    fn down(&self) -> Option<Self>;
}

impl Movement4Directions for V2usize {
    fn right(&self) -> Option<Self> {
        Some(V2usize::new(self.x + 1, self.y))
    }
    fn up(&self) -> Option<Self> {
        Some(V2usize::new(self.x, self.y + 1))
    }
    fn left(&self) -> Option<Self> {
        if self.x > 0 {
            Some(V2usize::new(self.x - 1, self.y))
        } else {
            None
        }
    }
    fn down(&self) -> Option<Self> {
        if self.y > 1 {
            Some(V2usize::new(self.x, self.y - 1))
        } else {
            None
        }
    }
}

impl IterateNeighbours for V2usize {
    fn neighbours(&self) -> Vec<Self> {
        let mut results = vec![];
        if let Some(a) = self.right() {
            results.push(a);
        }
        if let Some(a) = self.up() {
            results.push(a);
        }
        if let Some(a) = self.left() {
            results.push(a);
        }
        if let Some(a) = self.down() {
            results.push(a);
        }
        results
    }
}
