use std::{
    fmt::Display,
    io::{BufRead, BufReader},
    iter,
};

use super::{
    geometric_traits::{IterateNeighbours, IterateNeighboursContext, Movement4Directions},
    line_iterator::LineIterator,
    linear_index::LinearIndex,
    vector::{Vector, V2},
};

#[derive(Clone)]
pub struct ArrayNd<const N: usize, T> {
    pub data: Vec<T>,
    pub dims: [usize; N],
}
impl<const N: usize, T> LinearIndex<Vector<N, usize>> for ArrayNd<N, T> {
    fn index(&self, i: Vector<N, usize>) -> Option<usize> {
        if self.is_in_bounds(i) {
            let mut result = 0;
            for j in (0..N).rev() {
                result *= self.dims[j];
                result += i.values[j];
            }
            Some(result)
        } else {
            None
        }
    }

    fn unindex(&self, mut i: usize) -> Option<Vector<N, usize>> {
        let mut result = Vector::new([0; N]);
        for j in 0..N {
            result.values[j] = i % self.dims[j];
            i /= self.dims[j];
        }
        Some(result)
    }

    fn is_in_bounds(&self, i: Vector<N, usize>) -> bool {
        i.values.iter().zip(self.dims).all(|(&a, b)| a < b)
    }
}
impl<const N: usize, T> LinearIndex<Vector<N, i32>> for ArrayNd<N, T> {
    fn index(&self, i: Vector<N, i32>) -> Option<usize> {
        if self.is_in_bounds(i) {
            let mut result = 0;
            for j in (0..N).rev() {
                result *= self.dims[j];
                result += i.values[j] as usize;
            }
            Some(result)
        } else {
            None
        }
    }

    fn unindex(&self, mut i: usize) -> Option<Vector<N, i32>> {
        let mut result = Vector::new([0i32; N]);
        for j in 0..N {
            result.values[j] = (i % self.dims[j]).try_into().unwrap();
            i /= self.dims[j];
        }
        Some(result)
    }

    fn is_in_bounds(&self, i: Vector<N, i32>) -> bool {
        i.values
            .iter()
            .zip(self.dims)
            .all(|(&a, b)| a >= 0 && (a as usize) < b)
    }
}

impl<const N: usize, T: Copy + PartialEq> ArrayNd<N, T> {
    pub fn replace_all(&mut self, from: &T, to: &T) {
        self.data.iter_mut().for_each(|x| {
            if x == from {
                *x = *to;
            }
        })
    }
}

impl<const N: usize, T> ArrayNd<N, T> {
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

    pub fn find(&self, item: &T) -> Option<Vector<N, usize>>
    where
        T: PartialEq,
    {
        match self.data.iter().position(|x| x == item) {
            Some(index) => self.unindex(index),
            None => None,
        }
    }

    pub fn map<F, U>(&self, f: F) -> ArrayNd<N, U>
    where
        F: Fn(&T) -> U,
    {
        let data = self.data.iter().map(f).collect();

        ArrayNd::<N, U> {
            data,
            dims: self.dims,
        }
    }
}

impl<const N: usize, T> ArrayNd<N, T> {
    pub fn line_iter(&self, p0: Vector<N, i32>, p1: Vector<N, i32>) -> LineIterator<N> {
        LineIterator::new(p0, p1)
    }
}

impl<const N: usize, T> ArrayNd<N, T> {
    pub fn iter_values_in_line(
        &'_ self,
        p0: Vector<N, i32>,
        p1: Vector<N, i32>,
    ) -> impl Iterator<Item = &'_ T> {
        // self.line_iter(p0, p1).filter_map(|p| self.get(p))
        self.line_iter(p0, p1).map(|p| self.get(p).unwrap())
    }
}

impl<const N: usize, T: Copy> ArrayNd<N, T> {
    pub fn draw_line(&mut self, p0: Vector<N, i32>, p1: Vector<N, i32>, v: T) {
        for p in self.line_iter(p0, p1) {
            self.set(p, v);
        }
    }
}

// NOTE(lubo): Array2d
pub type Array2d<T> = ArrayNd<2, T>;
impl<T> Array2d<T> {
    pub fn width(&self) -> usize {
        self.dims[0]
    }
    pub fn height(&self) -> usize {
        self.dims[1]
    }
}

impl<T: Display> Display for Array2d<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in (0..self.height()).rev() {
            for x in 0..self.width() {
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
            dims: [width, height],
        }
    }
}

impl<T: Copy> Array2d<T> {
    pub fn new(width: usize, height: usize, default: T) -> Self {
        Self {
            data: iter::repeat(default).take(width * height).collect(),
            dims: [width, height],
        }
    }

    pub fn shift_n_rows_down(&mut self, n: usize, default: T) {
        self.data.drain(..self.width() * n);
        self.data.extend(
            iter::repeat(default)
                .take(self.width() * n)
                .collect::<Vec<T>>(),
        );
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
