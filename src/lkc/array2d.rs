use std::{
    fmt::Display,
    io::{BufRead, BufReader},
    iter,
};

use super::{
    linear_index::LinearIndex,
    v2::{V2i32, V2usize, V2},
};

#[derive(Clone)]
pub struct Array2D<T> {
    pub data: Vec<T>,
    pub width: usize,
    pub height: usize,
}

impl<T: Display> Display for Array2D<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                write!(f, "{}", self.get(V2::new(x, y)).unwrap())?;
            }
            writeln!(f)?;
        }

        write!(f, "")
    }
}

impl Array2D<char> {
    pub fn from_buffer<R: std::io::Read>(reader: BufReader<R>) -> Self {
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

impl<T> LinearIndex<V2usize> for Array2D<T> {
    fn index(&self, i: V2usize) -> Option<usize> {
        if i.x < self.width && i.y < self.height {
            Some(i.y * self.width + i.x)
        } else {
            None
        }
    }

    fn unindex(&self, i: usize) -> Option<V2usize> {
        let x = i % self.width;
        let y = i / self.width;
        if x < self.width && y < self.height {
            Some(V2usize::new(x, y))
        } else {
            None
        }
    }
}

impl<T> LinearIndex<V2i32> for Array2D<T> {
    fn index(&self, i: V2i32) -> Option<usize> {
        if i.x >= 0 && i.y >= 0 && (i.x as usize) < self.width && (i.y as usize) < self.height {
            Some((i.y as usize) * self.width + (i.x as usize))
        } else {
            None
        }
    }

    fn unindex(&self, i: usize) -> Option<V2i32> {
        let x = i % self.width;
        let y = i / self.width;
        if x < self.width && y < self.height {
            Some(V2i32::new(x as i32, y as i32))
        } else {
            None
        }
    }
}

impl<T: Copy> Array2D<T> {
    pub fn new(width: usize, height: usize, default: T) -> Self {
        Self {
            data: iter::repeat(default).take(width * height).collect(),
            width,
            height,
        }
    }
}

impl<T> Array2D<T> {
    pub fn set<I>(&mut self, p: I, v: T) -> bool
    where
        Self: LinearIndex<I>,
    {
        match self.get_mut(p) {
            Some(a) => {
                *a = v;
                true
            }
            None => false,
        }
    }

    pub fn get_mut<I>(&mut self, p: I) -> Option<&mut T>
    where
        Self: LinearIndex<I>,
    {
        match self.index(p) {
            Some(index) => Some(&mut self.data[index]),
            None => None,
        }
    }

    pub fn get<I>(&self, p: I) -> Option<&T>
    where
        Self: LinearIndex<I>,
    {
        match self.index(p) {
            Some(index) => Some(&self.data[index]),
            None => None,
        }
    }

    pub fn find(&self, item: &T) -> Option<V2usize>
    where
        T: PartialEq,
    {
        match self.data.iter().position(|x| x == item) {
            Some(index) => self.unindex(index),
            None => None,
        }
    }

    pub fn map<F, U>(&self, f: F) -> Array2D<U>
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

impl<T: Copy> Array2D<T> {
    pub fn draw_line(&mut self, p0: V2i32, p1: V2i32, v: T) {
        let dir = p1 - p0;

        let mut step_options = vec![];
        match dir.x.cmp(&0) {
            std::cmp::Ordering::Less => {
                step_options.push(V2::new(-1, 0));
                match dir.y.cmp(&0) {
                    std::cmp::Ordering::Less => {
                        step_options.push(V2::new(0, -1));
                        step_options.push(V2::new(-1, -1));
                    }
                    std::cmp::Ordering::Equal => (),
                    std::cmp::Ordering::Greater => {
                        step_options.push(V2::new(0, 1));
                        step_options.push(V2::new(-1, 1));
                    }
                };
            }
            std::cmp::Ordering::Equal => {
                match dir.y.cmp(&0) {
                    std::cmp::Ordering::Less => step_options.push(V2::new(0, -1)),
                    std::cmp::Ordering::Equal => (),
                    std::cmp::Ordering::Greater => step_options.push(V2::new(0, 1)),
                };
            }
            std::cmp::Ordering::Greater => {
                step_options.push(V2::new(1, 0));
                match dir.y.cmp(&0) {
                    std::cmp::Ordering::Less => {
                        step_options.push(V2::new(0, -1));
                        step_options.push(V2::new(1, -1));
                    }
                    std::cmp::Ordering::Equal => (),
                    std::cmp::Ordering::Greater => {
                        step_options.push(V2::new(0, 1));
                        step_options.push(V2::new(1, 1));
                    }
                };
            }
        };

        let mut at = p0;
        self.set(at, v);
        while at != p1 {
            let delta = p1 - at;
            let best_step = *step_options
                .iter()
                .max_by_key(|step| delta.inner(**step))
                .unwrap();
            at += best_step;
            self.set(at, v);
        }
    }
}
