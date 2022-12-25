use std::{
    fmt::Display,
    io::{BufRead, BufReader},
    iter,
};

use super::{
    geometric_traits::{IterateNeighbours, IterateNeighboursContext, Movement4Directions},
    line_iterator::{LineIterator, LineIterator2},
    linear_index::LinearIndex,
    vector::{V2i32, V2usize, V2},
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
                write!(f, "{}", self.get(V2::from_xy(x, y)).unwrap())?;
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
        if i.x() < self.width && i.y() < self.height {
            Some(i.y() * self.width + i.x())
        } else {
            None
        }
    }

    fn unindex(&self, i: usize) -> Option<V2usize> {
        let x = i % self.width;
        let y = i / self.width;
        if x < self.width && y < self.height {
            Some(V2usize::from_xy(x, y))
        } else {
            None
        }
    }

    fn is_in_bounds(&self, i: V2usize) -> bool {
        i.x() < self.width && i.y() < self.height
    }
}

impl<T> LinearIndex<V2i32> for Array2d<T> {
    fn index(&self, i: V2i32) -> Option<usize> {
        if i.x() >= 0
            && i.y() >= 0
            && (i.x() as usize) < self.width
            && (i.y() as usize) < self.height
        {
            Some((i.y() as usize) * self.width + (i.x() as usize))
        } else {
            None
        }
    }

    fn unindex(&self, i: usize) -> Option<V2i32> {
        let x = i % self.width;
        let y = i / self.width;
        if x < self.width && y < self.height {
            Some(V2i32::from_xy(x as i32, y as i32))
        } else {
            None
        }
    }

    fn is_in_bounds(&self, i: V2i32) -> bool {
        i.x() >= 0
            && i.y() >= 0
            && i.x() < self.width.try_into().unwrap()
            && i.y() < self.height.try_into().unwrap()
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

    pub fn shift_n_rows_down(&mut self, n: usize, default: T) {
        self.data.drain(..self.width * n);
        self.data.extend(
            iter::repeat(default)
                .take(self.width * n)
                .collect::<Vec<T>>(),
        );
    }
}

impl<T: Copy + PartialEq> Array2d<T> {
    pub fn replace_all(&mut self, from: &T, to: &T) {
        self.data.iter_mut().for_each(|x| {
            if x == from {
                *x = *to;
            }
        })
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

impl<T> Array2d<T> {
    pub fn line_iter(&self, p0: V2i32, p1: V2i32) -> LineIterator2 {
        LineIterator::new(p0, p1)
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

impl<T> IterateNeighboursContext for Array2d<T> {}

impl<T: Movement4Directions + Copy, U> IterateNeighbours<Array2d<U>> for T
where
    Array2d<U>: LinearIndex<T>,
{
    fn neighbours(&self, _context: &Array2d<U>) -> Vec<Self> {
        let mut results = vec![];
        if let Some(a) = self.step_right() {
            if _context.is_in_bounds(a) {
                results.push(a);
            }
        }
        if let Some(a) = self.step_up() {
            if _context.is_in_bounds(a) {
                results.push(a);
            }
        }
        if let Some(a) = self.step_left() {
            if _context.is_in_bounds(a) {
                results.push(a);
            }
        }
        if let Some(a) = self.step_down() {
            if _context.is_in_bounds(a) {
                results.push(a);
            }
        }
        results
    }
}
