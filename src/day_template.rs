use std::{cmp::max, io::prelude::*, io::BufReader};

use crate::{Day, Problem};

impl Problem for Day<0> {
    fn solve_buffer<T, W>(reader: BufReader<T>, writer: &mut W)
    where
        T: std::io::Read,
        W: std::io::Write,
    {
        let mut result = 0;

        // let lines: Vec<_> = reader.lines().collect();

        for line in reader.lines().map(|x| x.unwrap()) {
            // writeln!("{}", line);

            result = max(result, line.len());
        }

        writeln!(writer, "Result: {}", result).unwrap();
    }
}
