use std::{cmp::max, io::prelude::*, io::BufReader};

use crate::{Day, Problem};

impl Problem for Day<0> {
    fn solve_buffer<T>(reader: BufReader<T>) -> Result<(), ()>
    where
        T: std::io::Read,
    {
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
