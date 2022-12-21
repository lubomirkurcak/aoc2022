use std::{
    iter::Sum,
    ops::{Add, Sub},
};

use super::math::{ExclusiveMax, InclusiveMin, Interval};

#[derive(Debug)]
pub struct IntervalSet<T> {
    pub intervals: Vec<std::ops::Range<T>>,
}

impl<T: Copy + Ord> IntervalSet<T> {
    pub fn new() -> Self {
        Self { intervals: vec![] }
    }

    // pub fn union_unordered(&mut self, interval: std::ops::Range<i32>) {
    //     for (index, a) in self.intervals.iter().enumerate() {
    //         if let Some(united) = a.interval_union(&interval) {
    //             println!("United {:?} and {:?}", a, interval);
    //             self.intervals.remove(index);
    //             self.union(united);
    //             return;
    //         }
    //     }
    //     println!("Added new interval {:?}", interval);
    //     self.intervals.push(interval);
    // }

    // pub fn bounds_unordered(&self) -> Option<std::ops::Range<i32>> {
    //     if self.intervals.len() > 0 {
    //         let min = *self
    //             .intervals
    //             .iter()
    //             .map(|a| a.inclusive_min())
    //             .min()
    //             .unwrap();
    //         let max = *self
    //             .intervals
    //             .iter()
    //             .map(|a| a.exclusive_max())
    //             .max()
    //             .unwrap();
    //         Some(min..max)
    //     } else {
    //         None
    //     }
    // }

    // pub fn intersect(&mut self, interval: std::ops::Range<i32>) {
    //     if *interval.inclusive_min() >= *interval.exclusive_max() {
    //         self.intervals.clear();
    //         return;
    //     }
    //     if self.intervals.is_empty() {
    //         return;
    //     }
    //     let index0 = match self
    //         .intervals
    //         .binary_search_by(|x| x.inclusive_min().cmp(interval.inclusive_min()))
    //     {
    //         Ok(value) => value,
    //         Err(value) => value,
    //     };
    //     let index1 = match self
    //         .intervals
    //         .binary_search_by(|x| x.exclusive_max().cmp(interval.exclusive_max()))
    //     {
    //         Ok(value) => value,
    //         Err(value) => value,
    //     };
    //     // NOTE(lubo): We are subregion of a single segment
    //     if index0 > index1 {
    //         self.intervals.clear();
    //         self.intervals.push(interval);
    //         return;
    //     }
    //     // NOTE(lubo): Remove segments that definitely fall outside interval
    //     if index1 + 1 < self.intervals.len() {
    //         self.intervals.drain(index1 + 1..);
    //     }
    //     if index0 > 1 {
    //         self.intervals.drain(..index0 - 1);
    //     }
    //     let index1 = index1 - index0;
    //     let index0 = 0;
    //     todo!();
    // }

    pub fn intersect(&mut self, interval: std::ops::Range<T>) {
        self.intervals = self
            .intervals
            .iter()
            .filter_map(|x| x.interval_intersection(&interval))
            .collect();
    }

    pub fn union(&mut self, interval: std::ops::Range<T>) {
        if *interval.inclusive_min() >= *interval.exclusive_max() {
            return;
        }

        if self.intervals.is_empty() {
            self.intervals.push(interval);
            return;
        }

        let index0 = match self
            .intervals
            .binary_search_by(|x| x.inclusive_min().cmp(interval.inclusive_min()))
        {
            Ok(value) => value,
            Err(value) => value,
        };
        let index1 = match self
            .intervals
            .binary_search_by(|x| x.exclusive_max().cmp(interval.exclusive_max()))
        {
            Ok(value) => value,
            Err(value) => value,
        };

        if index0 > index1 {
            // NOTE(lubo): Already included
            return;
        }

        if index0 < index1 {
            // NOTE(lubo): We can definitely remove n = (index1 - index0) segments.
            // Segments to definitely remove:
            //  1. index0
            //  2. index0 + 1
            //  ...
            //  n. index0 + n - 1
            self.intervals.drain(index0..index1);
        }

        // NOTE(lubo): Either
        // 1. add new segment (+1 total)
        // 2. join left segment
        // 3. join right segment
        // 4. join both (-1 total)
        let index = index0;

        if index > 0 {
            let pre = self.intervals[index - 1].interval_union(&interval);
            if let Some(mut interval) = pre {
                if index < self.intervals.len() {
                    let all_three = self.intervals[index].interval_union(&interval);
                    if let Some(all_three) = all_three {
                        interval = all_three;
                        self.intervals.remove(index);
                    }
                }

                self.intervals[index - 1] = interval;
                return;
            }
        }

        if index < self.intervals.len() {
            let post = self.intervals[index].interval_union(&interval);
            if let Some(post) = post {
                self.intervals[index] = post;
                return;
            }
        }

        self.intervals.insert(index, interval);
    }
}

impl<T: Copy + Add<Output = T> + Sub<Output = T> + Sum> IntervalSet<T> {
    pub fn measure(&self) -> T {
        self.intervals
            .iter()
            .map(|x| *x.exclusive_max() - *x.inclusive_min())
            .sum()
    }

    pub fn bounds(&self) -> Option<std::ops::Range<T>> {
        let count = self.intervals.len();
        if count > 0 {
            Some(*self.intervals[0].inclusive_min()..*self.intervals[count - 1].exclusive_max())
        } else {
            None
        }
    }

    pub fn negation_within_bounds(&self) -> Self {
        let count = self.intervals.len();

        let mut negated = vec![];

        for i in 0..count - 1 {
            negated.push(*self.intervals[i].exclusive_max()..*self.intervals[i + 1].inclusive_min())
        }

        Self { intervals: negated }
    }
}
