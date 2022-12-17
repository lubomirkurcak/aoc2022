use std::io::{BufRead, BufReader};

use super::{linear_index::LinearIndex, v2::V2usize};

#[derive(Clone)]
pub struct Array2D<T> {
    pub data: Vec<T>,
    pub width: usize,
    pub height: usize,
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

impl<T> LinearIndex<Array2D<T>, V2usize> for Array2D<T> {
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

impl<T> Array2D<T> {
    pub fn set(&mut self, p: V2usize, v: T) -> bool {
        match self.get_mut(p) {
            Some(a) => {
                *a = v;
                true
            }
            None => false,
        }
    }

    pub fn get_mut(&mut self, p: V2usize) -> Option<&mut T> {
        match self.index(p) {
            Some(index) => Some(&mut self.data[index]),
            None => None,
        }
    }

    pub fn get(&self, p: V2usize) -> Option<&T> {
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
