use std::io::BufReader;

use crate::{
    lkc::{
        arraynd::Array2d,
        explore::{Exploration, ExploreSignals},
        sketch::QueueBag,
    },
    Day, Problem,
};

impl Problem for Day<1201> {
    fn solve_buffer<T, W>(reader: BufReader<T>, writer: &mut W)
    where
        T: std::io::Read,
        W: std::io::Write,
    {
        let mut map = Array2d::from_buffer(reader);
        let start = map.find(&'S').unwrap();
        let end = map.find(&'E').unwrap();
        map.set(start, 'a');
        map.set(end, 'z');

        let mut distances = map.map(|_| 999999999);
        distances.set(start, 0);

        let mut result = -1;
        let mut exp = Exploration::new(map, distances);
        exp.explore_avoid_identical::<_, _, QueueBag<_>>(
            start,
            |p, _map, distances| {
                if p == &end {
                    result = *distances.get(*p).unwrap();
                    ExploreSignals::ReachedGoal
                } else {
                    ExploreSignals::Explore
                }
            },
            |p, n, map, distances| {
                let from = *map.get(*p).unwrap() as i32;
                let to = *map.get(*n).unwrap() as i32;
                let accessible = from + 1 >= to;
                if accessible {
                    let dist_n = *distances.get(*n).unwrap();
                    let dist_p = *distances.get(*p).unwrap();
                    let relaxed = dist_p + 1;
                    if relaxed < dist_n {
                        distances.set(*n, relaxed);
                    }
                    if relaxed > dist_n {
                        dbg!(dist_n);
                        dbg!(dist_p);

                        panic!("THIS CANT HAPPEN!!!");
                    }
                }
                accessible
            },
        );

        writeln!(writer, "{}", result).unwrap();
    }
}

impl Problem for Day<1202> {
    fn solve_buffer<T, W>(reader: BufReader<T>, writer: &mut W)
    where
        T: std::io::Read,
        W: std::io::Write,
    {
        let mut map = Array2d::from_buffer(reader);
        let start = map.find(&'S').unwrap();
        let end = map.find(&'E').unwrap();
        map.set(start, 'a');
        map.set(end, 'z');

        let mut distances = map.map(|_| 999999999);
        distances.set(end, 0);

        let mut result = -1;
        let mut exp = Exploration::new(map, distances);
        exp.explore_avoid_identical::<_, _, QueueBag<_>>(
            end,
            |p, map, distances| {
                if *map.get(*p).unwrap() == 'a' {
                    result = *distances.get(*p).unwrap();
                    ExploreSignals::ReachedGoal
                } else {
                    ExploreSignals::Explore
                }
            },
            |p, n, map, distances| {
                let from = *map.get(*p).unwrap() as i32;
                let to = *map.get(*n).unwrap() as i32;
                let accessible = from - 1 <= to;
                if accessible {
                    let dist_n = *distances.get(*n).unwrap();
                    let dist_p = *distances.get(*p).unwrap();
                    let relaxed = dist_p + 1;
                    if relaxed < dist_n {
                        distances.set(*n, relaxed);
                    }
                }
                accessible
            },
        );

        writeln!(writer, "{}", result).unwrap();
    }
}
