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
pub struct Array2d<T> {
    pub data: Vec<T>,
    pub width: usize,
    pub height: usize,
}

impl<T: Display> Display for Array2d<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in (0..self.height).rev() {
            for x in 0..self.width {
                write!(f, "{}", self.get(V2::new(x, y)).unwrap())?;
            }
            writeln!(f)?;
        }

        write!(f, "")
    }
}

impl Array2d<char> {
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

impl<T> LinearIndex<V2usize> for Array2d<T> {
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

impl<T> LinearIndex<V2i32> for Array2d<T> {
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

impl<T: Copy> Array2d<T> {
    pub fn new(width: usize, height: usize, default: T) -> Self {
        Self {
            data: iter::repeat(default).take(width * height).collect(),
            width,
            height,
        }
    }
}

impl<T> Array2d<T> {
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

    pub fn map<F, U>(&self, f: F) -> Array2d<U>
    where
        F: Fn(&T) -> U,
    {
        let data = self.data.iter().map(f).collect();

        Array2d::<U> {
            data,
            width: self.width,
            height: self.height,
        }
    }
}

pub struct Array2dLineIterator {
    step_options: Vec<V2i32>,
    at: V2i32,
    end: V2i32,
}

impl Iterator for Array2dLineIterator {
    type Item = V2i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.step_options.is_empty() {
            let dir = self.end - self.at;
            match dir.x.cmp(&0) {
                std::cmp::Ordering::Less => {
                    self.step_options.push(V2::new(-1, 0));
                    match dir.y.cmp(&0) {
                        std::cmp::Ordering::Less => {
                            self.step_options.push(V2::new(0, -1));
                            self.step_options.push(V2::new(-1, -1));
                        }
                        std::cmp::Ordering::Equal => (),
                        std::cmp::Ordering::Greater => {
                            self.step_options.push(V2::new(0, 1));
                            self.step_options.push(V2::new(-1, 1));
                        }
                    };
                }
                std::cmp::Ordering::Equal => {
                    match dir.y.cmp(&0) {
                        std::cmp::Ordering::Less => self.step_options.push(V2::new(0, -1)),
                        std::cmp::Ordering::Equal => (),
                        std::cmp::Ordering::Greater => self.step_options.push(V2::new(0, 1)),
                    };
                }
                std::cmp::Ordering::Greater => {
                    self.step_options.push(V2::new(1, 0));
                    match dir.y.cmp(&0) {
                        std::cmp::Ordering::Less => {
                            self.step_options.push(V2::new(0, -1));
                            self.step_options.push(V2::new(1, -1));
                        }
                        std::cmp::Ordering::Equal => (),
                        std::cmp::Ordering::Greater => {
                            self.step_options.push(V2::new(0, 1));
                            self.step_options.push(V2::new(1, 1));
                        }
                    };
                }
            }

            Some(self.at)
        } else if self.at != self.end {
            let delta = self.end - self.at;
            let best_step = *self
                .step_options
                .iter()
                .max_by_key(|step| delta.inner(**step))
                .unwrap();
            self.at += best_step;

            Some(self.at)
        } else {
            None
        }
    }
}

impl<T> Array2d<T> {
    pub fn line_iter(&self, p0: V2i32, p1: V2i32) -> Array2dLineIterator {
        Array2dLineIterator {
            at: p0,
            end: p1,
            step_options: vec![],
        }
    }
}

impl<T> Array2d<T> {
    pub fn iter_values_in_line(&'_ self, p0: V2i32, p1: V2i32) -> impl Iterator<Item = &'_ T> {
        // self.line_iter(p0, p1).filter_map(|p| self.get(p))
        self.line_iter(p0, p1).map(|p| self.get(p).unwrap())
    }
}

impl<T: Copy> Array2d<T> {
    pub fn draw_line(&mut self, p0: V2i32, p1: V2i32, v: T) {
        for p in self.line_iter(p0, p1) {
            self.set(p, v);
        }
    }
}
