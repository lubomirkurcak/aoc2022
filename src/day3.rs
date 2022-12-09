use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::prelude::*,
    io::BufReader,
    path::Path,
};

use crate::{Day, Problem};

fn item_priority(item: char) -> usize {
    match item {
        'a'..='z' => item as usize - 'a' as usize + 1,
        'A'..='Z' => item as usize - 'A' as usize + 27,
        _ => 0,
    }
}

impl Problem for Day<3> {
    fn solve_file<P: AsRef<Path> + Copy>(path: P) -> Result<(), ()> {
        let file = match File::open(path) {
            Ok(it) => it,
            Err(_) => return Err(()),
        };

        let mut priority_sum = 0;
        for line in BufReader::new(file).lines() {
            let line = match line {
                Ok(line) => line,
                Err(_) => return Err(()),
            };

            let left = &line[0..line.len() / 2];
            let right = &line[line.len() / 2..];

            let mut collision_table = [0; 64];

            for char in left.chars() {
                let item_id = item_priority(char);
                collision_table[item_id] = 1;
            }

            for char in right.chars() {
                let item_id = item_priority(char);
                if collision_table[item_id] != 0 {
                    priority_sum += item_id;
                    break;
                }
            }
        }

        println!("Total duplicate priority sum: {}", priority_sum);

        // Part 2

        let mut badge_sum = 0;

        let file = match File::open(path) {
            Ok(it) => it,
            Err(_) => return Err(()),
        };

        let mut lines = BufReader::new(file).lines();
        while let (Some(Ok(line1)), Some(Ok(line2)), Some(Ok(line3))) =
            (lines.next(), lines.next(), lines.next())
        {
            let mut set = HashSet::new();
            let mut set2 = HashSet::new();

            for char in line1.chars() {
                set.insert(char);
            }

            for char in line2.chars() {
                if set.contains(&char) {
                    set2.insert(char);
                }
            }

            for char in line3.chars() {
                if set2.contains(&char) {
                    badge_sum += item_priority(char);
                    break;
                }
            }
        }

        println!("Badge sum: {}", badge_sum);

        Ok(())
    }
}
