use std::{io::prelude::*, io::BufReader, ops::Index};

use crate::{Day, Problem};
use lk_math::{
    arraynd::Array2d,
    explore::{Exploration, ExploreSignals},
    geometric_traits::IterateNeighbours,
    math::Gcd,
    sketch::{QueueBag, StackBag},
    vector::{V2i32, Vector},
};

struct BlizzardMap {
    map: Array2d<char>,
    left: Vec<V2i32>,
    up: Vec<V2i32>,
    right: Vec<V2i32>,
    down: Vec<V2i32>,

    cache: Vec<Array2d<char>>,
}

impl BlizzardMap {
    fn from_map(mut map: Array2d<char>) -> Self {
        let right = map.find_all(&'>');
        let up = map.find_all(&'^');
        let left = map.find_all(&'<');
        let down = map.find_all(&'v');

        map.replace_all(&'>', &'.');
        map.replace_all(&'^', &'.');
        map.replace_all(&'<', &'.');
        map.replace_all(&'v', &'.');

        let mut blizz = Self {
            map,
            right,
            left,
            up,
            down,
            cache: vec![],
        };

        blizz.cache = blizz.precalc_states();

        blizz
    }

    fn at_time(&self, t: i32) -> &Array2d<char> {
        let cycle_length = self.cycle_length();
        let t = t % cycle_length;
        &self.cache[t as usize]
    }

    fn cycle_length(&self) -> i32 {
        let width = (self.map.width() - 2) as i32;
        let height = (self.map.height() - 2) as i32;
        i32::gcd(width, height)
    }

    fn precalc_states(&self) -> Vec<Array2d<char>> {
        let mut cache = vec![];
        let cycle_length = self.cycle_length();
        for cycle in 0..cycle_length {
            cache.push(self.calc_at_time(cycle));
        }
        cache
    }

    fn calc_at_time(&self, t: i32) -> Array2d<char> {
        let width = (self.map.width() - 2) as i32;
        let height = (self.map.height() - 2) as i32;
        let cycle_length = self.cycle_length();
        let t = t % cycle_length;

        let mut map = self.map.clone();

        for p in self.right.iter() {
            let x = ((p.x() - 1) + t) % width + 1;
            let y = p.y();
            let p = Vector::from_xy(x, y);
            map.set(p, '#');
        }

        for p in self.left.iter() {
            let x = ((p.x() - 1) - t) % width + 1;
            let y = p.y();
            let p = Vector::from_xy(x, y);
            map.set(p, '#');
        }

        for p in self.up.iter() {
            let x = p.x();
            let y = ((p.y() - 1) - t) % height + 1;
            let p = Vector::from_xy(x, y);
            map.set(p, '#');
        }

        for p in self.down.iter() {
            let x = p.x();
            let y = ((p.y() - 1) + t) % height + 1;
            let p = Vector::from_xy(x, y);
            map.set(p, '#');
        }

        map
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Point {
    p: V2i32,
    t: i32,
}

impl IterateNeighbours<Array2d<char>> for Point {
    fn neighbours(&self, context: &Array2d<char>) -> Vec<Self> {
        self.p
            .neighbours(context)
            .into_iter()
            .map(|x| Self {
                p: x,
                t: self.t + 1,
            })
            .collect()
    }
}

impl Problem for Day<24> {
    fn solve_buffer<T, W>(reader: BufReader<T>, writer: &mut W)
    where
        T: std::io::Read,
        W: std::io::Write,
    {
        let map = Array2d::from_buffer(reader);

        println!("Map size: {}x{}", map.width(), map.height());

        let start = map.find(&'.').unwrap();
        let end = map.find_last(&'.').unwrap();
        let blizz = BlizzardMap::from_map(map);
        let awdlijalwijd = blizz.at_time(100);

        println!("{}", awdlijalwijd);

        let mut result = None;
        let mut exp = Exploration::new(blizz.map.clone(), blizz);
        exp.explore::<_, _, QueueBag<_>>(
            Point { p: start, t: 0 },
            |p, map, blizz| {
                if p.p.x() > 3 || p.p.y() > 3 {
                    println!("{:?}", p);
                }

                if p.p.y() + 1 == map.height().try_into().unwrap() {
                    result = Some(p.t);
                    ExploreSignals::ReachedGoal
                } else {
                    ExploreSignals::Explore
                }
            },
            |p, n, map, blizz| {
                let map_at_time = blizz.at_time(n.t);
                let n_standing_on = map_at_time.get(n.p).unwrap();
                n_standing_on == &'.'
            },
        );

        println!("Found in {result:?} steps.");

        writeln!(writer, "{:?}", result).unwrap();
    }
}
