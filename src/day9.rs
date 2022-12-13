use std::{
    cmp::max,
    collections::HashSet,
    io::prelude::*,
    io::BufReader,
    ops::{Add, AddAssign, Sub, SubAssign},
};

use crate::{Day, Problem};

// #[derive(AddAssign, SubAssign)]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Pos<T> {
    x: T,
    y: T,
}

impl<T> Pos<T> {
    fn new(x: T, y: T) -> Pos<T> {
        Pos { x, y }
    }
}

impl<T: Default> Default for Pos<T> {
    fn default() -> Self {
        Self::new(T::default(), T::default())
    }
}

impl<T: Add + std::ops::Add<Output = T>> Add for Pos<T> {
    type Output = Pos<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Pos {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T: AddAssign> AddAssign for Pos<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T: Sub + Sub<Output = T>> Sub for Pos<T> {
    type Output = Pos<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Pos {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T: SubAssign> SubAssign for Pos<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl Pos<i32> {
    fn right() -> Pos<i32> {
        Pos { x: 1, y: 0 }
    }
    fn up() -> Pos<i32> {
        Pos { x: 0, y: 1 }
    }
    fn left() -> Pos<i32> {
        Pos { x: -1, y: 0 }
    }
    fn down() -> Pos<i32> {
        Pos { x: 0, y: -1 }
    }
}

struct RopeLink<T> {
    p: Pos<T>,
    trail: HashSet<Pos<T>>,
    tail: Option<Box<RopeLink<T>>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MovementDir {
    None,
    R,
    U,
    L,
    D,
    UR,
    UL,
    DL,
    DR,
}

impl RopeLink<i32> {
    fn with_tail_length(tail_length: usize) -> Self {
        let initial_p = Pos::default();
        Self {
            p: initial_p,
            trail: HashSet::from([initial_p]),
            tail: match tail_length {
                0 => None,
                _ => Some(Box::new(RopeLink::with_tail_length(tail_length - 1))),
            },
        }
    }

    fn get_nth(&self, n: usize) -> &Self {
        match n {
            0 => self,
            _ => self.tail.as_ref().unwrap().get_nth(n - 1),
        }
    }

    fn get_last(&self) -> &Self {
        match self.tail.as_ref() {
            Some(tail) => tail.get_last(),
            None => self,
        }
    }

    fn delta(dir: MovementDir) -> Pos<i32> {
        match dir {
            MovementDir::None => Pos::default(),
            MovementDir::R => Pos::right(),
            MovementDir::U => Pos::up(),
            MovementDir::L => Pos::left(),
            MovementDir::D => Pos::down(),
            MovementDir::UR => Pos::up() + Pos::right(),
            MovementDir::UL => Pos::up() + Pos::left(),
            MovementDir::DR => Pos::down() + Pos::right(),
            MovementDir::DL => Pos::down() + Pos::left(),
        }
    }

    fn move_n_times(&mut self, dir: MovementDir, repeat: usize) {
        for _ in 0..repeat {
            self.move_1_time(dir);
        }
    }

    // +----+----+----+
    // |    |    |    |
    // +----+----+----+
    // |    |    |    |
    // +----+----+----+
    // |    |    |    |
    // +----+----+----+

    // +----+----+----+----+----+
    // |    |    |    |    |    |
    // +----+----+----+----+----+
    // |    |    |    |    |    |
    // +----+----+----+----+----+
    // |    |    |    |    |    |
    // +----+----+----+----+----+
    // |    |    |    |    |    |
    // +----+----+----+----+----+
    // |    |    |    |    |    |
    // +----+----+----+----+----+
    fn move_1_time(&mut self, dir: MovementDir) {
        self.p += Self::delta(dir);

        if let Some(tail) = self.tail.as_mut() {
            let delta = self.p - tail.p;

            let tail_dir = match (delta.x, delta.y) {
                (2, 0) => MovementDir::R,
                (-2, 0) => MovementDir::L,
                (0, 2) => MovementDir::U,
                (0, -2) => MovementDir::D,
                (2, 2) | (2, 1) | (1, 2) => MovementDir::UR,
                (-2, 2) | (-2, 1) | (-1, 2) => MovementDir::UL,
                (2, -2) | (2, -1) | (1, -2) => MovementDir::DR,
                (-2, -2) | (-2, -1) | (-1, -2) => MovementDir::DL,
                _ => MovementDir::None,
            };

            if tail_dir != MovementDir::None {
                tail.move_1_time(tail_dir);
            }
        }

        self.trail.insert(self.p);
    }
}

impl Problem for Day<9> {
    fn solve_buffer<T>(reader: BufReader<T>) -> Result<(), ()>
    where
        T: std::io::Read,
    {
        let mut result = 0;

        let mut rope = RopeLink::<i32>::with_tail_length(9);

        for line in reader.lines() {
            let line = match line {
                Ok(line) => line,
                Err(_) => return Err(()),
            };

            let args = line.split_whitespace().collect::<Vec<&str>>();
            let count = args[1].parse().unwrap();
            match args[0] {
                "R" => rope.move_n_times(MovementDir::R, count),
                "U" => rope.move_n_times(MovementDir::U, count),
                "L" => rope.move_n_times(MovementDir::L, count),
                "D" => rope.move_n_times(MovementDir::D, count),
                _ => panic!(),
            }

            result = max(result, line.len());
        }

        println!(
            "First child touched grass at {} places",
            rope.get_nth(1).trail.len()
        );
        println!("Last child at {} places", rope.get_last().trail.len());

        Ok(())
    }
}
