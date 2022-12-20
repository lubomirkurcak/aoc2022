use std::{collections::HashSet, io::BufReader};

use crate::{
    lkc::{array2d::Array2D, geometric_traits::IterateNeighbours, v2::V2usize},
    Day, Problem,
};

enum ExplorationLocation {
    Specific(V2usize),
    Type(char),
}

struct Exploration {
    map: Array2D<char>,
    distance: Array2D<Option<i32>>,
    open: Vec<V2usize>,
    closed: HashSet<V2usize>,
    goal: ExplorationLocation,
    traversable: fn(char, char) -> bool,
}

impl Exploration {
    fn from(
        map: &Array2D<char>,
        start: V2usize,
        goal: ExplorationLocation,
        traversable: fn(char, char) -> bool,
    ) -> Self {
        let map = map.clone();

        let mut distance = map.map(|_x| None);
        distance.set(start, Some(0));

        Self {
            map,
            open: vec![start],
            closed: HashSet::new(),
            distance,
            goal,
            traversable,
        }
    }

    fn open_tiles(&self, p: V2usize) -> impl Iterator<Item = V2usize> {
        let mut open = vec![];

        if let Some(&from) = self.map.get(p) {
            for p in p.neighbours(&()) {
                if let Some(&to) = self.map.get(p) {
                    if (self.traversable)(from, to) {
                        open.push(p);
                    }
                }
            }
        }

        open.into_iter()
    }

    fn explore(&mut self) -> i32 {
        loop {
            let next = self.open.drain(..1).next();
            if let Some(tile) = next {
                self.closed.insert(tile);

                if match self.goal {
                    ExplorationLocation::Specific(location) => tile == location,
                    ExplorationLocation::Type(t) => *self.map.get(tile).unwrap() == t,
                } {
                    return self.distance.get(tile).unwrap().unwrap();
                }

                let distance0 = self.distance.get(tile).unwrap().unwrap();
                for tile in self.open_tiles(tile) {
                    if !self.closed.contains(&tile) {
                        if !self.open.contains(&tile) {
                            self.open.push(tile);
                        }
                        let distance1 = self.distance.get_mut(tile).unwrap();
                        *distance1 = match distance1 {
                            Some(distance1) => Some(std::cmp::min(*distance1, distance0 + 1)),
                            None => Some(distance0 + 1),
                        }
                    }
                }
            }
        }
    }
}

impl Problem for Day<12> {
    fn solve_buffer<T, W>(reader: BufReader<T>, writer: &mut W)
    where
        T: std::io::Read,
        W: std::io::Write,
    {
        let mut map = Array2D::from_buffer(reader);
        let start = map.find(&'S').unwrap();
        let end = map.find(&'E').unwrap();
        map.set(start, 'a');
        map.set(end, 'z');

        let mut a = Exploration::from(
            &map,
            start,
            ExplorationLocation::Specific(end),
            |from, to| from as i32 + 1 >= to as i32,
        );

        let mut b = Exploration::from(&map, end, ExplorationLocation::Type('a'), |from, to| {
            from as i32 - 1 <= to as i32
        });

        writeln!(writer, "Distance to end 1: {}", a.explore()).unwrap();
        writeln!(writer, "Distance to end 2: {}", b.explore()).unwrap();
    }
}
