use std::{io::prelude::*, io::BufReader};

use crate::{Day, Problem};

#[derive(Debug)]
enum Instruction {
    Noop,
    Addx(i32),
}

impl Instruction {
    // TODO(lubo): Implement From
    fn from_str(line: &str) -> Instruction {
        let args = line.split_whitespace().collect::<Vec<&str>>();
        match args[0] {
            "noop" => Instruction::Noop,
            "addx" => Instruction::Addx(args[1].parse().unwrap()),

            _ => panic!(),
        }
    }

    fn latency(&self) -> usize {
        match *self {
            Instruction::Noop => 1,
            Instruction::Addx(_) => 2,
        }
    }
}

impl Problem for Day<10> {
    fn solve_buffer<T, W>(reader: BufReader<T>, writer: &mut W)
    where
        T: std::io::Read,
        W: std::io::Write,
    {
        let mut cycle = 0;
        let mut x = 1;

        let interests = (20..=220).step_by(40);
        let mut instructions = reader.lines().map(|x| Instruction::from_str(&x.unwrap()));

        let mut result = 0;

        for interest in interests {
            for instruction in instructions.by_ref() {
                let mut interest_done = false;

                if cycle + instruction.latency() >= interest {
                    result += interest as i32 * x;
                    writeln!(writer, "At the point of interest of cycle {}, 'x' is {}. We are adding {} to reach cumulative result of {}", interest, x, interest as i32 * x, result).unwrap();
                    interest_done = true;
                }

                cycle += instruction.latency();
                match instruction {
                    Instruction::Noop => (),
                    Instruction::Addx(val) => x += val,
                }

                writeln!(
                    writer,
                    "after {:?} 'x' has value {}, at cycle {}",
                    instruction, x, cycle
                )
                .unwrap();

                if interest_done {
                    break;
                }
            }
        }

        writeln!(writer, "Result: {}", result).unwrap();
    }
}
