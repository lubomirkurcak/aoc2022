use std::{
    collections::HashMap,
    io::{BufRead, BufReader},
};

use crate::{day17::Rock, Problem};

pub struct Day17Optimized<const C: usize>;

struct PatternInfo {
    itr: usize,
    ffr: isize,
}

impl PatternInfo {
    fn new(itr: usize, ffr: isize) -> Self {
        Self { itr, ffr }
    }
}

impl<const C: usize> Problem for Day17Optimized<C> {
    fn solve_buffer<T, W>(reader: BufReader<T>, writer: &mut W)
    where
        T: std::io::Read,
        W: std::io::Write,
    {
        let wind = reader
            .lines()
            .map(|x| x.unwrap())
            .flat_map(|x| x.chars().collect::<Vec<_>>())
            .map(|x| match x {
                '>' => true,
                '<' => false,
                _ => panic!(),
            })
            .collect::<Vec<_>>();
        let wind_count = wind.len();
        let mut wind = wind.iter().cycle();

        let heights = [
            Rock::height(0),
            Rock::height(1),
            Rock::height(2),
            Rock::height(3),
            Rock::height(4),
        ];
        let rocks = [
            Rock::construct_mask(0) << 2,
            Rock::construct_mask(1) << 2,
            Rock::construct_mask(2) << 2,
            Rock::construct_mask(3) << 2,
            Rock::construct_mask(4) << 2,
        ];
        let mut heights = heights.iter().cycle();
        let mut rocks = rocks.iter().cycle();

        let left_side = Rock::left_side_mask();
        let right_side = Rock::right_side_mask();

        const TOWER_SIZE: usize = 4 * 1024;
        const TOWER_KEEP: usize = 512;
        const TOWER_ALMOSTFULL: usize = TOWER_SIZE - 10;
        const TOWER_ALMOSTFULLI: isize = TOWER_ALMOSTFULL as isize;
        const TOWER_DELETE: usize = TOWER_SIZE - TOWER_KEEP;
        const TOWER_DELETEI: isize = TOWER_DELETE as isize;
        let mut tower = [0u8; TOWER_SIZE];
        tower[0] = 127;

        let mut pattern_skip_done = false;
        let mut pattern_map = HashMap::new();

        let mut result = -1;
        let mut first_free_row = 1;
        let mut iteration = 0;
        while iteration < C {
            let mut rock = *rocks.next().unwrap();
            let height = *heights.next().unwrap();

            let mut row = first_free_row;

            if *wind.next().unwrap() {
                rock <<= 1;
            } else {
                rock >>= 1;
            }

            if *wind.next().unwrap() {
                if rock & right_side == 0 {
                    rock <<= 1;
                }
            } else {
                // NOTE(lubo): Collision not possible on the left side yet.
                rock >>= 1;
            }

            if *wind.next().unwrap() {
                if rock & right_side == 0 {
                    rock <<= 1;
                }
            } else {
                #[allow(clippy::collapsible_else_if)]
                if rock & left_side == 0 {
                    rock >>= 1;
                }
            }

            if *wind.next().unwrap() {
                if rock & right_side == 0 {
                    rock <<= 1;
                }
            } else {
                #[allow(clippy::collapsible_else_if)]
                if rock & left_side == 0 {
                    rock >>= 1;
                }
            }

            loop {
                // NOTE(lubo): First possible collision
                row -= 1;

                unsafe {
                    let at = tower.as_ptr().offset(row) as *const u32;
                    if *at & rock != 0 {
                        row += 1;
                        let at = tower.as_ptr().offset(row) as *mut u32;
                        *at |= rock;
                        first_free_row = std::cmp::max(first_free_row, row + height as isize);

                        if first_free_row > TOWER_ALMOSTFULLI {
                            first_free_row -= TOWER_DELETEI;
                            result += TOWER_DELETEI;

                            tower.copy_within(TOWER_DELETE.., 0);
                            tower[TOWER_KEEP..].fill(0);
                        }

                        break;
                    }
                }

                let w = *wind.next().unwrap();
                if w {
                    if rock & right_side == 0 {
                        rock <<= 1;
                    }
                } else {
                    #[allow(clippy::collapsible_else_if)]
                    if rock & left_side == 0 {
                        rock >>= 1;
                    }
                }

                unsafe {
                    let at = tower.as_ptr().offset(row) as *const u32;
                    if *at & rock != 0 {
                        match w {
                            true => rock >>= 1,
                            false => rock <<= 1,
                        };
                    }
                }
            }

            if !pattern_skip_done && iteration > 0 && iteration % (wind_count * 5) == 0 {
                // NOTE(lubo): Print

                unsafe {
                    let a = *(tower.as_ptr().offset(first_free_row - 16) as *const u128);
                    if let std::collections::hash_map::Entry::Vacant(e) = pattern_map.entry(a) {
                        e.insert(PatternInfo::new(iteration, result + first_free_row));
                    } else {
                        let pattern = pattern_map.get(&a).unwrap();

                        let itr_delta = iteration - pattern.itr;
                        let ffr_delta = result + first_free_row - pattern.ffr;

                        println!(
                            "Repeated pattern {}  @ iteration {} itr_delta {} ffr_delta {}",
                            a, iteration, itr_delta, ffr_delta,
                        );

                        let mut skipped_cycles = 0;
                        while iteration + itr_delta < C {
                            iteration += itr_delta;
                            result += ffr_delta;
                            skipped_cycles += 1;
                        }

                        println!("Skipped {} iterations!", skipped_cycles * itr_delta);
                        pattern_skip_done = true;
                    }
                }

                #[cfg(disabled)]
                {
                    println!("+--------------+");
                    for x in tower[std::cmp::max(0, first_free_row - 20) as usize
                        ..first_free_row as usize + 3]
                        .iter()
                        .rev()
                    {
                        // println!("{:#09b}", x);
                        print!("|");
                        for bit_id in 0..7 {
                            if x & (1 << bit_id) > 0 {
                                print!("[]");
                            } else {
                                print!("  ");
                            }
                        }
                        println!("|");
                    }
                    println!("+--------------+");
                }
            }

            iteration += 1;
        }

        result += first_free_row;

        println!("{}", result);
        writeln!(writer, "{}", result).unwrap();
    }
}
