use std::{io::prelude::*, io::BufReader};

use crate::{
    lkc::{
        array2d::Array2d,
        v2::{V2i32, V2},
    },
    Problem,
};

struct Rock;
impl Rock {
    fn width(t: usize) -> i32 {
        match t {
            0 => 4,
            1 => 3,
            2 => 3,
            3 => 1,
            4 => 2,
            _ => panic!(),
        }
    }
    fn height(t: usize) -> i32 {
        match t {
            0 => 1,
            1 => 3,
            2 => 3,
            3 => 4,
            4 => 2,
            _ => panic!(),
        }
    }
    fn construct(t: usize) -> Vec<(V2i32, V2i32)> {
        match t {
            0 => vec![(V2::new(0, 0), V2::new(3, 0))],
            1 => vec![
                (V2::new(0, 1), V2::new(2, 1)),
                (V2::new(1, 0), V2::new(1, 2)),
            ],
            2 => vec![
                (V2::new(0, 0), V2::new(2, 0)),
                (V2::new(2, 0), V2::new(2, 2)),
            ],
            3 => vec![(V2::new(0, 0), V2::new(0, 3))],
            4 => vec![
                (V2::new(0, 0), V2::new(1, 0)),
                (V2::new(0, 1), V2::new(1, 1)),
            ],
            _ => panic!(),
        }
    }
}

pub struct Day17<const C: usize>;

impl<const C: usize> Problem for Day17<C> {
    fn solve_buffer<T, W>(reader: BufReader<T>, writer: &mut W)
    where
        T: std::io::Read,
        W: std::io::Write,
    {
        let check = |map: &Array2d<char>, blueprint: &Vec<_>, p| {
            blueprint
                .iter()
                .map(|line: &(V2i32, V2i32)| {
                    map.iter_values_in_line(line.0 + p, line.1 + p)
                        .all(|c| c == &'.')
                })
                .all(|x| x)
        };

        let draw = |map: &mut Array2d<char>, blueprint: &Vec<(V2i32, V2i32)>, p| {
            blueprint.iter().for_each(|line| {
                map.draw_line(line.0 + p, line.1 + p, '#');
            })
        };

        let wind = reader
            .lines()
            .map(|x| x.unwrap())
            .flat_map(|x| x.chars().collect::<Vec<_>>())
            .map(|x| match x {
                '>' => 1,
                '<' => -1,
                _ => panic!(),
            })
            .collect::<Vec<_>>();
        let mut wind = wind.iter().cycle();

        let mut map = Array2d::new(7, 100, '.');

        let mut first_free_row = 0;
        for rock_type in (0..5).cycle().take(C) {
            let mut p = V2::new(2, first_free_row + 3);
            let blueprint = Rock::construct(rock_type);
            let width = Rock::width(rock_type);
            let height = Rock::height(rock_type);

            assert!(check(&map, &blueprint, p));

            loop {
                let x = (p.x + *wind.next().unwrap()).clamp(0, 7 - width);
                if p.x != x && check(&map, &blueprint, V2::new(x, p.y)) {
                    p.x = x;
                }

                if p.y > 0 && check(&map, &blueprint, V2::new(p.x, p.y - 1)) {
                    p.y -= 1;
                } else {
                    draw(&mut map, &blueprint, p);
                    first_free_row = std::cmp::max(first_free_row, p.y + height);
                    break;
                }
            }

            println!("{}", map);
        }
        println!("{}", first_free_row);

        writeln!(writer, "{}", 0).unwrap();
    }
}
