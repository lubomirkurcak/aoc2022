use super::{
    geometric_traits::{IterateNeighbours, IterateNeighboursContext},
    sketch::Bag,
};
use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    hash::Hash,
};

// pub struct ExplorationBuilder<P, C, H, N>
// where
//     H: Fn(&P) -> i32,
// {
//     context: Option<C>,
//     avoid_identical: Option<HashSet<P>>,
//     heuristic: Option<H>,
//     neighbours: Option<N>,
// }
//
// impl<P, C, H, N> ExplorationBuilder<P, C, H, N>
// where
//     H: Fn(&P) -> i32,
// {
//     pub fn new() -> Self {
//         Self {
//             context: None,
//             avoid_identical: None,
//             heuristic: None,
//             neighbours: None,
//         }
//     }
//
//     pub fn with_context(self, context, extra_data: C) -> Self {
//         self.context = Some(context);
//         self
//     }
//
//     pub fn with_neighbours(self, neighbours: N) -> Self {
//         self.neighbours = Some(neighbours);
//         self
//     }
//
//     pub fn with_heuristic(self, heuristic: H) -> Self {
//         self.heuristic = Some(heuristic);
//         self
//     }
//
//     pub fn avoid_identical(self, context, extra_data: C) -> Self {
//         self.avoid_identical = Some(HashSet::new());
//         self
//     }
//
//     pub fn build(self) -> Exploration<P, C> {
//         let exp = Exploration::<P, C>::new(self.context.unwrap_or(()));
//         exp
//     }
//     // pub fn from_arraynd()
//     // pub fn with_distance()
// }

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
    fn compare_values(k: &Self::K, a: &Self::V, b: &Self::V) -> Option<Ordering>;
}

#[derive(Debug)]
pub struct Exploration<P: IterateNeighbours<S>, S: IterateNeighboursContext, D> {
    pub context: S,
    phantom: std::marker::PhantomData<P>,
    pub extra_data: D,
}

impl<P: Clone + Copy, S: IterateNeighboursContext, D> Exploration<P, S, D>
where
    P: IterateNeighbours<S> + Hash + Eq,
{
    pub fn new(context: S, extra_data: D) -> Self {
        Self {
            context,
            phantom: std::marker::PhantomData,
            extra_data,
        }
    }

    pub fn explore<F, G, B: Bag<P>>(&mut self, start: P, mut goal: G, mut filter_neighbours: F)
    where
        F: FnMut(&P, &P, &mut S, &mut D) -> bool,
        G: FnMut(&P, &mut S, &mut D) -> ExploreSignals,
    {
        self.explore_advanced::<_, _, _, B>(
            start,
            (),
            |p, _data, context, extra_data| goal(p, context, extra_data),
            |p, n, _data, context, extra_data| filter_neighbours(p, n, context, extra_data),
        )
    }

    // NOTE(lubo): Uses a hashset to avoid identical states
    pub fn explore_avoid_identical<F, G, B: Bag<P>>(
        &mut self,
        start: P,
        mut goal: G,
        mut filter_neighbours: F,
    ) where
        F: FnMut(&P, &P, &mut S, &mut D) -> bool,
        G: FnMut(&P, &mut S, &mut D) -> ExploreSignals,
    {
        self.explore_advanced::<_, _, _, B>(
            start,
            HashSet::new(),
            |p, data, context, extra_data| {
                if data.contains(p) {
                    ExploreSignals::Skip
                } else {
                    data.insert(*p);
                    goal(p, context, extra_data)
                }
            },
            |p, n, data, context, extra_data| {
                !data.contains(n) && filter_neighbours(p, n, context, extra_data)
            },
        )
    }

    pub fn explore_advanced<T, F, G, B: Bag<P>>(
        &mut self,
        start: P,
        mut data: T,
        mut goal: G,
        mut filter_neighbours: F,
    ) where
        F: FnMut(&P, &P, &mut T, &mut S, &mut D) -> bool,
        G: FnMut(&P, &mut T, &mut S, &mut D) -> ExploreSignals,
    {
        let mut open = B::new();
        open.put(start);
        while !open.is_empty() {
            let p = open.get().unwrap();
            // let mut open = vec![start];
            // while !open.is_empty() {
            //     let p = open.pop().unwrap();

            match goal(&p, &mut data, &mut self.context, &mut self.extra_data) {
                ExploreSignals::ReachedGoal => break,
                ExploreSignals::Explore => (),
                ExploreSignals::Skip => continue,
            }

            for n in p.neighbours(&self.context) {
                if filter_neighbours(&p, &n, &mut data, &mut self.context, &mut self.extra_data) {
                    open.put(n);
                    // open.push(n);
                }
            }
        }
    }
}

impl<P, S, D> Exploration<P, S, D>
where
    P: IterateNeighbours<S> + PointKeyValue,
    P: Clone + Copy + Hash + Eq,
    S: IterateNeighboursContext,
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
    pub fn explore_avoid_worse<F, G, B: Bag<P>>(
        &mut self,
        start: P,
        mut goal: G,
        mut filter_neighbours: F,
    ) where
        F: FnMut(&P, &P, &mut S, &mut D) -> bool,
        G: FnMut(&P, &mut S, &mut D) -> ExploreSignals,
    {
        self.explore_advanced::<_, _, _, B>(
            start,
            HashMap::<<P as PointKeyValue>::K, <P as PointKeyValue>::V>::new(),
            |p, data, context, extra_data| {
                let k = p.get_key();
                let v = p.get_value();

                if let Some(&old_v) = data.get(&k) {
                    if let Some(ordering) = P::compare_values(&k, &v, &old_v) {
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

                goal(p, context, extra_data)
            },
            |p, n, data, context, extra_data| {
                let k = n.get_key();
                let v = n.get_value();

                if let Some(&old_v) = data.get(&k) {
                    if P::compare_values(&k, &v, &old_v) == Some(Ordering::Less) {
                        return false;
                    }
                }

                filter_neighbours(p, n, context, extra_data)
            },
        )
    }
}
