use std::{collections::HashSet, io::prelude::*, io::BufReader};

use crate::{Day, Problem};

#[derive(Clone)]
struct Array2D<T> {
    data: Vec<T>,
    width: usize,
    height: usize,
}

impl Array2D<char> {
    fn from_buffer<R: std::io::Read>(reader: BufReader<R>) -> Self {
        let lines: Vec<_> = reader.lines().map(|x| x.unwrap()).collect();
        let height = lines.len();
        let width = lines.iter().map(|x| x.len()).max().unwrap();
        assert!(lines.iter().all(|x| x.len() == width));
        let data = lines.concat().chars().collect();

        Self {
            data,
            width,
            height,
        }
    }
}

trait LinearIndex<T> {
    type I;
    fn index(&self, i: I) -> Option<usize>;
    fn unindex(&self, i: usize) -> Option<I>;
}

impl<T> LinearIndex<Array2D<T>> for Array2D<T> {
    type I = (usize, usize);

    fn index(&self, i: I) -> Option<usize> {
        if i.0 < self.width && i.1 < self.height {
            Some(i.1 * self.width + i.0)
        } else {
            None
        }
    }

    fn unindex(&self, i: usize) -> Option<I> {
        let x = i % self.width;
        let y = i / self.width;
        if x < self.width && y < self.height {
            Some((x, y))
        } else {
            None
        }
    }
}

type I = (usize, usize);

trait Movement4Directions
where
    Self: std::marker::Sized,
{
    fn right(&self) -> Option<Self>;
    fn up(&self) -> Option<Self>;
    fn left(&self) -> Option<Self>;
    fn down(&self) -> Option<Self>;
}

impl Movement4Directions for I {
    fn right(&self) -> Option<Self> {
        Some((self.0 + 1, self.1))
    }
    fn up(&self) -> Option<Self> {
        Some((self.0, self.1 + 1))
    }
    fn left(&self) -> Option<Self> {
        if self.0 > 0 {
            Some((self.0 - 1, self.1))
        } else {
            None
        }
    }
    fn down(&self) -> Option<Self> {
        if self.1 > 1 {
            Some((self.0, self.1 - 1))
        } else {
            None
        }
    }
}

impl<T> Array2D<T> {
    fn set(&mut self, p: I, v: T) -> bool {
        match self.get_mut(p) {
            Some(a) => {
                *a = v;
                true
            }
            None => false,
        }
    }

    fn get_mut(&mut self, p: I) -> Option<&mut T> {
        match self.index(p) {
            Some(index) => Some(&mut self.data[index]),
            None => None,
        }
    }

    fn get(&self, p: I) -> Option<&T> {
        match self.index(p) {
            Some(index) => Some(&self.data[index]),
            None => None,
        }
    }

    fn find(&self, item: &T) -> Option<I>
    where
        T: PartialEq,
    {
        match self.data.iter().position(|x| x == item) {
            Some(index) => self.unindex(index),
            None => None,
        }
    }

    fn map<F, U>(&self, f: F) -> Array2D<U>
    where
        F: Fn(&T) -> U,
    {
        let data = self.data.iter().map(f).collect();

        Array2D::<U> {
            data,
            width: self.width,
            height: self.height,
        }
    }
}

enum ExplorationLocation {
    Specific(I),
    Type(char),
}

struct Exploration {
    map: Array2D<char>,
    // map_num: Array2D<i32>,
    distance: Array2D<Option<i32>>,
    open: Vec<I>,
    closed: HashSet<I>,
    // start: I,
    // end: I,
    goal: ExplorationLocation,
    traversable: fn(char, char) -> bool,
}

impl Exploration {
    fn from(
        map: &Array2D<char>,
        start: I,
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
            // start,
            // end,
            // map_num,
            distance,
            goal,
            traversable,
        }
    }

    fn open_tiles(&self, p: I) -> impl Iterator<Item = I> {
        let mut open = vec![];

        if let Some(&from) = self.map.get(p) {
            let mut try_add = |p| {
                if let Some(p) = p {
                    if let Some(&to) = self.map.get(p) {
                        if (self.traversable)(from, to) {
                            open.push(p);
                        }
                    }
                }
            };

            try_add(p.right());
            try_add(p.up());
            try_add(p.left());
            try_add(p.down());
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
