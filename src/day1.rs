use std::io::{prelude::*, BufReader};

use crate::{Day, Problem};

impl Problem for Day<1> {
    fn solve_buffer<T, W>(reader: BufReader<T>, writer: &mut W)
    where
        T: std::io::Read,
        W: std::io::Write,
    {
        let mut sums = vec![];
        let mut current = 0;

        for line in reader.lines().map(|x| x.unwrap()) {
            if line.is_empty() {
                sums.push(current);
                current = 0;
            } else {
                current += line.parse::<i32>().unwrap();
            }
        }

        sums.push(current);
        sums.sort();
        sums.reverse();
        let top1 = sums[0];
        let top3sum: i32 = sums.iter().take(3).sum();
        writeln!(writer, "TOP 1: {}", top1).unwrap();
        writeln!(writer, "TOP 3 SUM: {}", top3sum).unwrap();
    }
}
