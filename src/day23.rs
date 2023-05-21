#![allow(clippy::reversed_empty_ranges)]

use ndarray::{prelude::*, Zip};
use std::io::{BufReader, Read};

type Board = Array2<i8>;

fn parse(n: usize, x: &[u8]) -> Board {
    let pad_size: usize = n;
    let m: usize = n + 2 * pad_size;
    let a = Array::from_iter(x.iter().filter_map(|&b| match b {
        b'#' => Some(1),
        b'.' => Some(0),
        _ => None,
    }));
    let a = a.into_shape((n, n)).unwrap();

    let mut map = Board::from_elem((m, m), 0);
    let pad_size: i32 = pad_size.try_into().unwrap();
    map.slice_mut(s![pad_size..-pad_size, pad_size..-pad_size])
        .assign(&a);
    map
}

const PX: i8 = 1; // East
const PY: i8 = 2; // South
const NX: i8 = 4; // West
const NY: i8 = 8; // North
const OCC: i8 = 16; // Occupied

fn iterate(z: &mut Board, dir_order: &[i8]) -> bool {
    let m = z.dim().0;

    // TODO(lubo): Might be a good idea to cache these. (Optimized build probably already does but unoptimized unit test run struggles)
    let mut neigh_mask = Board::zeros((m - 2, m - 2));
    let mut proposition_dirs = Board::zeros((m, m));
    let mut proposition_ps = Board::zeros((m, m));
    let mut move_tos = Board::zeros((m, m));
    let mut move_froms = Board::zeros((m, m));

    let center = s![1..-1, 1..-1];
    let nx = s![1..-1, ..-2];
    let px = s![1..-1, 2..];
    let ny = s![..-2, 1..-1];
    let py = s![2.., 1..-1];

    // (y,x)
    // (-1,-1)
    let slice = &z.slice(s![..-2, ..-2]);
    neigh_mask = neigh_mask | (NX * slice);
    neigh_mask = neigh_mask | (NY * slice);

    // (-1,0)
    let slice = &z.slice(ny);
    neigh_mask = neigh_mask | (NY * slice);

    // (-1,1)
    let slice = &z.slice(s![..-2, 2..]);
    neigh_mask = neigh_mask | (PX * slice);
    neigh_mask = neigh_mask | (NY * slice);

    // (0,-1)
    let slice = &z.slice(nx);
    neigh_mask = neigh_mask | (NX * slice);

    // (0,0)
    let slice = &z.slice(center);
    neigh_mask = neigh_mask | (OCC * slice);

    // (0,1)
    let slice = &z.slice(px);
    neigh_mask = neigh_mask | (PX * slice);

    // (1,-1)
    let slice = &z.slice(s![2.., ..-2]);
    neigh_mask = neigh_mask | (NX * slice);
    neigh_mask = neigh_mask | (PY * slice);

    // (1,0)
    let slice = &z.slice(py);
    neigh_mask = neigh_mask | (PY * slice);

    // (1,1)
    let slice = &z.slice(s![2.., 2..]);
    neigh_mask = neigh_mask | (PX * slice);
    neigh_mask = neigh_mask | (PY * slice);

    let mut terminal = true;

    // NOTE(lubo): "First half" - pick the first available direction according to the problem definition
    let mut pdv = proposition_dirs.slice_mut(center);
    pdv.zip_mut_with(&neigh_mask, |x, &n| {
        if (n & OCC) > 0 {
            *x = OCC;

            if (n & (PX | PY | NX | NY)) > 0 {
                terminal = false;
                for dir in dir_order.iter() {
                    if (n & dir) == 0 {
                        *x |= *dir;
                        break;
                    }
                }
            }
        }
    });

    if terminal {
        return true;
    }

    // NOTE(lubo): "Second half"
    let mut ppv = proposition_ps.slice_mut(center);

    let pdv_px = proposition_dirs.slice(px);
    let pdv_py = proposition_dirs.slice(py);
    let pdv_nx = proposition_dirs.slice(nx);
    let pdv_ny = proposition_dirs.slice(ny);

    // NOTE(lubo): Tally guys who want to move...
    // ... west
    ppv.zip_mut_with(&pdv_px, |y, &x| {
        if (x & OCC) > 0 && (x & NX) > 0 {
            *y += 1;
        }
    });

    // ... north
    ppv.zip_mut_with(&pdv_py, |y, &x| {
        if (x & OCC) > 0 && (x & NY) > 0 {
            *y += 1;
        }
    });

    // ... east
    ppv.zip_mut_with(&pdv_nx, |y, &x| {
        if (x & OCC) > 0 && (x & PX) > 0 {
            *y += 1;
        }
    });

    // ... south
    ppv.zip_mut_with(&pdv_ny, |y, &x| {
        if (x & OCC) > 0 && (x & PY) > 0 {
            *y += 1;
        }
    });

    // TODO(lubo): Could we zip and update all of these in one go?!
    // NOTE(lubo): Move if there is only one proposal to that destination
    Zip::from(&mut move_tos.slice_mut(center))
        .and(&mut move_froms.slice_mut(px))
        .and(&ppv)
        .and(&pdv_px)
        .for_each(|to, from, &y, &x| {
            if (x & OCC) > 0 && (x & NX) > 0 && y == 1 {
                *to += 1;
                *from += 1;
            }
        });

    Zip::from(&mut move_tos.slice_mut(center))
        .and(&mut move_froms.slice_mut(py))
        .and(&ppv)
        .and(&pdv_py)
        .for_each(|to, from, &y, &x| {
            if (x & OCC) > 0 && (x & NY) > 0 && y == 1 {
                *to += 1;
                *from += 1;
            }
        });

    Zip::from(&mut move_tos.slice_mut(center))
        .and(&mut move_froms.slice_mut(nx))
        .and(&ppv)
        .and(&pdv_nx)
        .for_each(|to, from, &y, &x| {
            if (x & OCC) > 0 && (x & PX) > 0 && y == 1 {
                *to += 1;
                *from += 1;
            }
        });

    Zip::from(&mut move_tos.slice_mut(center))
        .and(&mut move_froms.slice_mut(ny))
        .and(&ppv)
        .and(&pdv_ny)
        .for_each(|to, from, &y, &x| {
            if (x & OCC) > 0 && (x & PY) > 0 && y == 1 {
                *to += 1;
                *from += 1;
            }
        });

    azip!((z in z, &to in &move_tos, &from in &move_froms) *z += to - from);

    false
}

use crate::{Day, Problem};
use lk_math::{aabb::Aabb, vector::Vector};

impl Problem for Day<2301> {
    fn solve_buffer<T, W>(mut reader: BufReader<T>, writer: &mut W)
    where
        T: std::io::Read,
        W: std::io::Write,
    {
        println!("North: {NY} (NY)  South: {PY} (PY)  West: {NX} (NX)  East: {PX} (PX)");

        let mut buffer = vec![];
        let n_squared = reader.read_to_end(&mut buffer).unwrap();
        let n = f64::sqrt(n_squared as f64) as usize;
        let mut a = parse(n, &buffer);
        let mut dir_order = vec![NY, PY, NX, PX];
        let steps = 10;
        for _ in 0..steps {
            iterate(&mut a, &dir_order);
            dir_order.rotate_left(1);
        }

        let elves = a
            .indexed_iter()
            .filter_map(|(i, &x)| {
                if x > 0 {
                    Some(Vector::from_xy(i.1, i.0))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        let elves_aabb = Aabb::covering(&elves).unwrap();
        let aabb_dim = elves_aabb.dim();
        let aabb_area = (aabb_dim.x() + 1) * (aabb_dim.y() + 1);
        let elves_count = a.iter().filter(|&&x| x > 0).count();
        let free_spaces = aabb_area - elves_count;

        println!("There are {free_spaces} free spaces in AABB");
        writeln!(writer, "{free_spaces}").unwrap();
    }
}

impl Problem for Day<2302> {
    fn solve_buffer<T, W>(mut reader: BufReader<T>, writer: &mut W)
    where
        T: std::io::Read,
        W: std::io::Write,
    {
        let mut buffer = vec![];
        let n_squared = reader.read_to_end(&mut buffer).unwrap();
        let n = f64::sqrt(n_squared as f64) as usize;
        let mut a = parse(n, &buffer);
        let mut step: usize = 0;

        let mut dir_order = vec![NY, PY, NX, PX];
        loop {
            if iterate(&mut a, &dir_order) {
                break;
            }
            dir_order.rotate_left(1);
            step += 1;
        }

        // NOTE(lubo): For some reason aoc wants one more than we get here.
        let step = step + 1;

        println!("Simulation terminates in {step} steps.");
        writeln!(writer, "{step}").unwrap();
    }
}
