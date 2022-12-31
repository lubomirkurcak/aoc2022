#![allow(clippy::reversed_empty_ranges)]

use ndarray::prelude::*;
use std::{io::prelude::*, io::BufReader};

const INPUT: &[u8] = include_bytes!("../in23.txt");
const N: usize = 74;

type Board = Array2<u8>;

fn parse(x: &[u8]) -> Board {
    let a = Array::from_iter(x.iter().filter_map(|&b| match b {
        b'#' => Some(1),
        b'.' => Some(0),
        _ => None,
    }));
    let a = a.into_shape((N, N)).unwrap();

    let pad_size = 2;
    let mut map = Board::from_elem(((N + 2 * pad_size), (N + 2 * pad_size)), 0);
    let pad_size: i32 = pad_size.try_into().unwrap();
    map.slice_mut(s![pad_size..-pad_size, pad_size..-pad_size])
        .assign(&a);
    map
}

fn iterate(z: &mut Board, scratch: &mut Board) {
    // compute number of neighbors
    let mut neigh = scratch.view_mut();
    neigh.fill(0);

    // NOTE(lubo): This is really clever, I like it!
    // But I'll have to actually have a (worst case) boundary pad of size N+1 on each side!
    // Too tired to do this now but can't wait for it!
    neigh += &z.slice(s![0..-2, 0..-2]);
    neigh += &z.slice(s![0..-2, 1..-1]);
    neigh += &z.slice(s![0..-2, 2..]);

    neigh += &z.slice(s![1..-1, 0..-2]);
    neigh += &z.slice(s![1..-1, 2..]);

    neigh += &z.slice(s![2.., 0..-2]);
    neigh += &z.slice(s![2.., 1..-1]);
    neigh += &z.slice(s![2.., 2..]);

    // birth where n = 3 and z[i] = 0,
    // survive where n = 2 || n = 3 and z[i] = 1
    let mut zv = z.slice_mut(s![1..-1, 1..-1]);

    // this is autovectorized amazingly well!
    zv.zip_mut_with(&neigh, |y, &n| *y = ((n == 3) || (n == 2 && *y > 0)) as u8);
}

fn turn_on_corners(z: &mut Board) {
    let n = z.nrows();
    let m = z.ncols();
    z[[1, 1]] = 1;
    z[[1, m - 2]] = 1;
    z[[n - 2, 1]] = 1;
    z[[n - 2, m - 2]] = 1;
}

fn render(a: &Board) {
    for row in a.rows() {
        for &x in row {
            if x > 0 {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

use crate::{Day, Problem};

impl Problem for Day<23> {
    fn solve_buffer<T, W>(reader: BufReader<T>, writer: &mut W)
    where
        T: std::io::Read,
        W: std::io::Write,
    {
        let mut result = 0;

        let mut a = parse(INPUT);
        let mut scratch = Board::zeros((N, N));
        let steps = 100;
        turn_on_corners(&mut a);
        for _ in 0..steps {
            iterate(&mut a, &mut scratch);
            render(&a);
            turn_on_corners(&mut a);
            render(&a);
        }
        render(&a);
        let alive = a.iter().filter(|&&x| x > 0).count();
        println!("After {} steps there are {} cells alive", steps, alive);

        writeln!(writer, "{}", result).unwrap();
    }
}
