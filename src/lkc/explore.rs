use super::geometric_traits::IterateNeighbours;
use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    hash::Hash,
};

pub enum ExploreSignals {
    ReachedGoal,
    Explore,
    Skip,
}

pub trait PointKeyValue {
    type K: Eq + Clone + Copy + Hash;
    type V: Eq + Clone + Copy + Hash;

    fn get_key(&self) -> Self::K;
    fn get_value(&self) -> Self::V;

    // NOTE(lubo): Less means WORSE and should NOT be explored.
    // NOTE(lubo): Greater means BETTER and SHOULD be explored.
    fn compare_values(a: &Self::V, b: &Self::V) -> Option<Ordering>;
}

#[derive(Debug)]
pub struct Exploration<P: IterateNeighbours, S> {
    pub structure: S,
    phantom: std::marker::PhantomData<P>,
}

impl<P: Clone + Copy, S> Exploration<P, S>
where
    P: IterateNeighbours<Context = Self> + Hash + Eq,
{
    pub fn new(structure: S) -> Self {
        Self {
            structure,
            phantom: std::marker::PhantomData,
        }
    }

    pub fn explore<F, G>(&self, start: P, mut goal: G, mut filter_neighbours: F)
    where
        F: FnMut(&P) -> bool,
        G: FnMut(&P) -> ExploreSignals,
    {
        self.explore_advanced(
            start,
            (),
            |p, _data| goal(p),
            |p, _data| filter_neighbours(p),
        )
    }

    // NOTE(lubo): Uses a hashset to avoid identical states
    pub fn explore_avoid_identical<F, G>(&self, start: P, mut goal: G, mut filter_neighbours: F)
    where
        F: FnMut(&P) -> bool,
        G: FnMut(&P) -> ExploreSignals,
    {
        self.explore_advanced(
            start,
            HashSet::new(),
            |p, data| {
                data.insert(*p);
                goal(p)
            },
            |p, data| !data.contains(p) && filter_neighbours(p),
        )
    }

    pub fn explore_advanced<T, F, G>(
        &self,
        start: P,
        mut data: T,
        mut goal: G,
        mut filter_neighbours: F,
    ) where
        F: FnMut(&P, &mut T) -> bool,
        G: FnMut(&P, &mut T) -> ExploreSignals,
    {
        // let mut open = StackBag::new();
        // open.put(start);
        // while !open.is_empty() {
        //     let p = open.get().unwrap();
        let mut open = vec![start];
        while !open.is_empty() {
            let p = open.pop().unwrap();

            match goal(&p, &mut data) {
                ExploreSignals::ReachedGoal => break,
                ExploreSignals::Explore => (),
                ExploreSignals::Skip => continue,
            }

            for neighbour in p.neighbours(self) {
                if filter_neighbours(&neighbour, &mut data) {
                    // open.put(neighbour);
                    open.push(neighbour);
                }
            }
        }
    }
}

impl<P: Clone + Copy, S> Exploration<P, S>
where
    P: IterateNeighbours<Context = Self> + Hash + Eq + PointKeyValue,
{
    // NOTE(lubo): Uses a hashmap that tracks the best 'value: V' for reach explored 'key: K'
    // 'V', 'K' and 'P::compare_values(a: &V, b &V)' need to be defined by user.
    //
    // For example, imagine you encounter 'point: P', which has 'key: K' defined as position, and 'value: V' as health.
    // We can drop this state if we previously encountered a state with the same position ('key: K') and more health ('value: V').
    //
    // In pseudocode:
    //
    // struct Point
    //   position: (i32,i32)
    //   health: i32
    //
    // impl PointKeyValue for Point
    //   get_key() => self.position
    //   get_value() => self.health
    //   Point::compare_values(a: &V, b &V) => Some(a < b)
    pub fn explore_avoid_worse<F, G>(&self, start: P, mut goal: G, mut filter_neighbours: F)
    where
        F: FnMut(&P) -> bool,
        G: FnMut(&P) -> ExploreSignals,
    {
        self.explore_advanced(
            start,
            HashMap::<<P as PointKeyValue>::K, <P as PointKeyValue>::V>::new(),
            |p, data| {
                let k = p.get_key();
                let v = p.get_value();

                if let Some(&old_v) = data.get(&k) {
                    if let Some(ordering) = P::compare_values(&v, &old_v) {
                        match ordering {
                            Ordering::Less => return ExploreSignals::Skip,
                            Ordering::Equal => (),
                            Ordering::Greater => {
                                data.insert(k, v);
                            }
                        }
                    }
                } else {
                    data.insert(k, v);
                }

                goal(p)
            },
            |p, data| {
                let k = p.get_key();
                let v = p.get_value();

                if let Some(&old_v) = data.get(&k) {
                    if P::compare_values(&v, &old_v) == Some(Ordering::Less) {
                        return false;
                    }
                }

                filter_neighbours(p)
            },
        )
    }
}
