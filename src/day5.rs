use std::{cmp::max, fs::File, io::prelude::*, io::BufReader, path::Path};

use crate::{Day, Problem};

impl Problem for Day<0> {
    fn solve_file<P: AsRef<Path>>(path: P) -> Result<(), ()> {
        let file = match File::open(path) {
            Ok(it) => it,
            Err(_) => return Err(()),
        };
        let reader = BufReader::new(file);

        let mut result = 0;

        // let results: Vec<_> = reader.lines().collect();

        for line in reader.lines() {
            let line = match line {
                Ok(line) => line,
                Err(_) => return Err(()),
            };

            result = max(result, line.len());
        }

        println!("Result: {}", result);

        Ok(())
    }
}
