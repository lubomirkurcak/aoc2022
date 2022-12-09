use std::{fs::File, io::prelude::*, io::BufReader, path::Path};

use crate::{Day, Problem};

impl Problem for Day<4> {
    fn solve_file<P: AsRef<Path>>(path: P) -> Result<(), ()> {
        let file = match File::open(path) {
            Ok(it) => it,
            Err(_) => return Err(()),
        };
        let reader = BufReader::new(file);

        // let results: Vec<_> = reader.lines().collect();
        let mut full_overlap_count = 0;
        let mut full_overlap_count_idea = 0;
        let mut some_overlap_count = 0;

        for line in reader.lines() {
            let line = match line {
                Ok(line) => line,
                Err(_) => return Err(()),
            };

            let split: Vec<&str> = line.split(',').collect();
            if let [a, b] = split[..] {
                let a: Vec<&str> = a.split('-').collect();

                if let [a0, a1] = a[..] {
                    let b: Vec<&str> = b.split('-').collect();

                    if let [b0, b1] = b[..] {
                        if let Ok(a0) = a0.parse::<i32>() {
                            if let Ok(a1) = a1.parse::<i32>() {
                                if let Ok(b0) = b0.parse::<i32>() {
                                    if let Ok(b1) = b1.parse::<i32>() {
                                        // println!("[{}-{}] vs [{}-{}]", a0, a1, b0, b1);

                                        if (a0 >= b0 && a1 <= b1) || (b0 >= a0 && b1 <= a1) {
                                            full_overlap_count += 1;
                                        }

                                        if (b0 - a0) * (b1 - a1) <= 0 {
                                            full_overlap_count_idea += 1;
                                        }

                                        if a1 >= b0 && a0 <= b1 {
                                            some_overlap_count += 1;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            } else {
                return Err(());
            }
        }

        println!("One fully overlaps other count: {}", full_overlap_count);
        println!(
            "One fully overlaps other count other formula: {}",
            full_overlap_count_idea
        );
        println!("Any overlap count: {}", some_overlap_count);

        Ok(())
    }
}
