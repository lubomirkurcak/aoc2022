use std::{io::prelude::*, io::BufReader, str::FromStr};

use crate::lkc::array2d::Array2d;
use crate::lkc::transformations::{Transform, Translation};

use crate::lkc::geometric_traits::{CoverObject, Movement4Directions};
use crate::lkc::vector::{Scalar, V2};
use crate::{lkc::aabb::Aabb2, Problem};

pub struct Day14<const C: bool>;

impl<const C: bool> Problem for Day14<C> {
    fn solve_buffer<T, W>(reader: BufReader<T>, writer: &mut W)
    where
        T: std::io::Read,
        W: std::io::Write,
    {
        let sand_coords = V2::from_xy(500, 0);
        let mut aabb = Aabb2::new(sand_coords, sand_coords);

        let lines = reader.lines().map(|x| x.unwrap()).collect::<Vec<_>>();
        for line in lines.iter() {
            for coords in line.split("->") {
                let coords = V2::<i32>::from_str(coords).unwrap();
                aabb.cover(&coords);
            }
        }

        let floor_y = aabb.max.y() + 2;
        let sand_height = floor_y;
        let floor_x_min = sand_coords.x() - sand_height - 1;
        let floor_x_max = sand_coords.x() + sand_height + 1;
        let floor_a = V2::from_xy(floor_x_min, floor_y);
        let floor_b = V2::from_xy(floor_x_max, floor_y);
        aabb.cover(&floor_a);
        aabb.cover(&floor_b);

        let map_dim = aabb.dim() + V2::from_xy(1, 1) + Scalar::new(4) * V2::from_xy(1, 1);
        let t = Translation::new(aabb.min - V2::from_xy(2, 2));

        let mut map = Array2d::new(map_dim.x() as usize, map_dim.y() as usize, '.');
        let sand_coords = t.inverse_transform(sand_coords);
        map.set(sand_coords, '+');
        let floor_a = t.inverse_transform(floor_a);
        let floor_b = t.inverse_transform(floor_b);
        if C {
            map.draw_line(floor_a, floor_b, '#');
        }

        for line in lines.iter() {
            let mut prev: Option<V2<_>> = None;
            for coords in line.split("->") {
                let coords = t.inverse_transform(V2::<i32>::from_str(coords).unwrap());
                if let Some(prev) = prev {
                    map.draw_line(prev, coords, '#');
                }
                prev = Some(coords);
            }
        }
        // println!("{}", map);

        let mut came_to_rest = 0;
        'outer: loop {
            let mut sand = sand_coords;
            loop {
                let a = sand.step_up().unwrap();
                match map.get(a) {
                    Some(atile) => {
                        let b = a.step_left().unwrap();
                        let c = a.step_right().unwrap();
                        if atile == &'.' {
                            sand = a;
                        } else if map.get(b).unwrap() == &'.' {
                            sand = b;
                        } else if map.get(c).unwrap() == &'.' {
                            sand = c;
                        } else {
                            came_to_rest += 1;
                            if sand == sand_coords {
                                break 'outer;
                            }
                            map.set(sand, 'o');
                            break;
                        }
                    }
                    None => break 'outer,
                }
            }
        }
        // println!("{}", map);
        println!("{} grains of sand came to rest.", came_to_rest);

        write!(writer, "{}", came_to_rest).unwrap();
    }
}
