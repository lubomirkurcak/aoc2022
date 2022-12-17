pub trait CoverObject<T> {
    fn cover(&mut self, object: &T);
}

// TODO(lubo): Is there a way to generalize distance?
pub trait ManhattanDistance<T, O> {
    fn manhattan_distance(&self, other: &Self) -> O;
}

pub trait EuclideanDistanceSquared<T, O> {
    fn euclidean_distance_squared(&self, other: &Self) -> O;
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
    // fn right() -> Option<Self>;
    // fn up() -> Option<Self>;
    // fn left() -> Option<Self>;
    // fn down() -> Option<Self>;

    fn step_right(&self) -> Option<Self>;
    fn step_up(&self) -> Option<Self>;
    fn step_left(&self) -> Option<Self>;
    fn step_down(&self) -> Option<Self>;
}

impl<T: Movement4Directions> IterateNeighbours for T {
    fn neighbours(&self) -> Vec<Self> {
        let mut results = vec![];
        if let Some(a) = self.step_right() {
            results.push(a);
        }
        if let Some(a) = self.step_up() {
            results.push(a);
        }
        if let Some(a) = self.step_left() {
            results.push(a);
        }
        if let Some(a) = self.step_down() {
            results.push(a);
        }
        results
    }
}
