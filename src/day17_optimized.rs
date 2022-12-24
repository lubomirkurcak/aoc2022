use std::{
    collections::{HashMap, HashSet},
    io::{BufRead, BufReader},
};

use crate::{day17::Rock, lkc::cli::Progress, Problem};

pub struct Day17Optimized<const C: usize>;

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
        println!("Wind length: {}", wind_count);
        let mut wind = wind.iter().cycle();

        let widths = [
            Rock::width(0),
            Rock::width(1),
            Rock::width(2),
            Rock::width(3),
            Rock::width(4),
        ];
        let heights = [
            Rock::height(0),
            Rock::height(1),
            Rock::height(2),
            Rock::height(3),
            Rock::height(4),
        ];
        let rocks = [
            Rock::construct_mask(0),
            Rock::construct_mask(1),
            Rock::construct_mask(2),
            Rock::construct_mask(3),
            Rock::construct_mask(4),
        ];
        let mut widths = widths.iter().cycle();
        let mut heights = heights.iter().cycle();
        let mut rocks = rocks.iter().cycle();

        let left_side = Rock::left_side_mask();
        let right_side = Rock::right_side_mask();

        // const TOWER_SIZE: usize = 128;
        const TOWER_SIZE: usize = 900 * 1024;
        const TOWER_KEEP: usize = 2 * 1024;
        const TOWER_ALMOSTFULL: usize = TOWER_SIZE - 10;
        const TOWER_ALMOSTFULLI: isize = TOWER_ALMOSTFULL as isize;
        const TOWER_DELETE: usize = TOWER_SIZE - TOWER_KEEP;
        const TOWER_DELETEI: isize = TOWER_DELETE as isize;
        let mut tower = [0u8; TOWER_SIZE];
        tower[0] = 127;

        let mut progress = Progress::new(C);

        let mut heights_at_input_cycle_same_record = 0;
        let mut heights_at_input_cycle = HashMap::new();

        let mut result = -1;
        let mut first_free_row = 1;
        for iteration in 0..C {
            let mut rock = *rocks.next().unwrap();
            let height = *heights.next().unwrap();
            rock <<= 2;

            let mut row = first_free_row + 3;

            if *wind.next().unwrap() {
                rock <<= 1;
            } else {
                rock >>= 1;
            }

            row -= 1;

            if *wind.next().unwrap() {
                if rock & right_side == 0 {
                    rock <<= 1;
                }
            } else {
                // NOTE(lubo): Collision not possible on the left side yet.
                rock >>= 1;
            }

            row -= 1;

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

            row -= 1;

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

                            progress.progress(iteration);
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

            if iteration % (wind_count * 5) == 0 {
                // NOTE(lubo): Print

                let mut heights = [0u8; 7];
                let mut heights_done = 0;
                for x in tower[..first_free_row as usize].iter().rev() {
                    heights_done |= x;

                    for (bit_id, height) in heights.iter_mut().enumerate() {
                        let mask = 1 << bit_id;
                        if heights_done & mask == 0 {
                            if x & mask > 0 {
                                heights_done |= mask;
                            } else {
                                *height += 1;
                            }
                        }
                    }

                    if heights_done & 0b1111111 == 0b1111111 {
                        break;
                    }
                }

                for h in heights.iter() {
                    if *h > 90 {
                        println!("HUUGGGGGGEEEEEEEEEEEE!!!!!! height {}", h);
                    } else if *h > 100 {
                        println!("Big height {}", h);
                    }
                }

                if let std::collections::hash_map::Entry::Vacant(e) =
                    heights_at_input_cycle.entry(heights)
                {
                    e.insert(1);
                    // println!("Inserted {:?}!", heights);
                } else {
                    *heights_at_input_cycle.get_mut(&heights).unwrap() += 1;
                    if *heights_at_input_cycle.get_mut(&heights).unwrap()
                        > heights_at_input_cycle_same_record
                    {
                        heights_at_input_cycle_same_record =
                            *heights_at_input_cycle.get_mut(&heights).unwrap();
                        println!(
                            "Matched {:?} a total of {} times @ iteration {}!",
                            heights,
                            heights_at_input_cycle.get_mut(&heights).unwrap(),
                            iteration
                        );
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
                }
            }
        }

        result += first_free_row;

        println!("{}", result);
        writeln!(writer, "{}", result).unwrap();
    }
}
