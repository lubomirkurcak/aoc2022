use std::io::{prelude::*, BufReader};

use crate::{Day, Problem};

impl Problem for Day<1> {
    fn solve_buffer<T>(reader: BufReader<T>) -> Result<(), ()>
    where
        T: std::io::Read,
    {
        let mut sums = vec![];
        let mut current = 0;

        for line in reader.lines() {
            let line = match line {
                Ok(line) => line,
                Err(_) => return Err(()),
            };

            if line.is_empty() {
                sums.push(current);
                current = 0;
            } else {
                current += match line.parse::<i32>() {
                    Ok(value) => value,
                    Err(_) => return Err(()),
                };
            }
        }
        sums.push(current);
        sums.sort_by(|a, b| b.cmp(a));
        let top1 = sums[0];
        let top3sum: i32 = sums.iter().take(3).sum();
        println!("TOP 1: {}", top1);
        println!("TOP 3 SUM: {}", top3sum);

        Ok(())
    }
}
