use std::{io::prelude::*, io::BufReader};

use crate::{Day, Problem};

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Noop,
    Addx(i32),
}

#[derive(Debug, Clone, Copy)]
struct ExecutingInstruction(Instruction, i32);

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

    fn latency(&self) -> i32 {
        match *self {
            Instruction::Noop => 1,
            Instruction::Addx(_) => 2,
        }
    }
}

impl Problem for Day<101> {
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
                    result += interest * x;
                    // writeln!(writer, "At the point of interest of cycle {}, 'x' is {}. We are adding {} to reach cumulative result of {}", interest, x, interest * x, result).unwrap();
                    interest_done = true;
                }

                cycle += instruction.latency();
                match instruction {
                    Instruction::Noop => (),
                    Instruction::Addx(val) => x += val,
                }

                if interest_done {
                    break;
                }
            }
        }

        writeln!(writer, "Result: {}", result).unwrap();
    }
}

impl Problem for Day<102> {
    fn solve_buffer<T, W>(reader: BufReader<T>, writer: &mut W)
    where
        T: std::io::Read,
        W: std::io::Write,
    {
        let mut x: i32 = 1;

        let mut current_instruction: Option<ExecutingInstruction> = None;

        let mut instructions = reader.lines().map(|x| Instruction::from_str(&x.unwrap()));

        for cycle in (0..240).step_by(1) {
            if let Some(xinstruction) = current_instruction {
                if cycle >= xinstruction.1 {
                    match xinstruction.0 {
                        Instruction::Noop => (),
                        Instruction::Addx(val) => x += val,
                    }
                    current_instruction = None;
                }
            }
            if current_instruction.is_none() {
                if let Some(instruction) = instructions.next() {
                    current_instruction = Some(ExecutingInstruction(
                        instruction,
                        cycle + instruction.latency(),
                    ));
                }
            }

            let pixel_x_position = cycle % 40;
            let pixel_char = match pixel_x_position - x {
                -1..=1 => '#',
                _ => '.',
            };

            write!(writer, "{}", pixel_char).unwrap();
            if pixel_x_position == 39 {
                writeln!(writer).unwrap();
            }
        }
    }
}
