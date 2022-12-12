use std::{fs::File, io::BufReader, path::Path};

use crate::{
    day3::{Day3CommonItemInCompartments, Day3CommonItemInGroups},
    day4::{Day4, OneFullyInsideAnotherOptimized, OneFullyInsideAnotherSimple, Overlap},
    day5::{CrateMover9000, CrateMover9001, Day5},
    day6::Day6,
};

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day_template;

fn main() {
    println!("Running tests!");

    ProblemRunner::<Day<0>>::run("in1.txt");
    ProblemRunner::<Day<1>>::run("in1.txt");
    ProblemRunner::<Day<2>>::run("in2.txt");
    ProblemRunner::<Day3CommonItemInCompartments>::run("in3.txt");
    ProblemRunner::<Day3CommonItemInGroups>::run("in3.txt");
    ProblemRunner::<Day4<OneFullyInsideAnotherSimple>>::run("in4.txt");
    ProblemRunner::<Day4<OneFullyInsideAnotherOptimized>>::run("in4.txt");
    ProblemRunner::<Day4<Overlap>>::run("in4.txt");
    ProblemRunner::<Day5<CrateMover9000>>::run("in5.txt");
    ProblemRunner::<Day5<CrateMover9001>>::run("in5.txt");
    ProblemRunner::<Day6<4>>::run("in6.txt");
    ProblemRunner::<Day6<14>>::run("in6.txt");
    // ProblemRunner::<Day<7>>::run("in7_small.txt");
    ProblemRunner::<Day<7>>::run("in7.txt");
    ProblemRunner::<Day<8>>::run("in8_small.txt");
    // ProblemRunner::<Day<8>>::run("in8.txt");
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum Outcome {
    Loss,
    Draw,
    Win,
}

struct Day<const T: usize> {}

trait Problem {
    fn solve_file<P: AsRef<Path> + Copy>(path: P) -> Result<(), ()> {
        let file = match File::open(path) {
            Ok(it) => it,
            Err(_) => return Err(()),
        };

        Self::solve_buffer(BufReader::new(file))
    }

    fn solve_buffer<T>(reader: BufReader<T>) -> Result<(), ()>
    where
        T: std::io::Read;
}

struct ProblemRunner<T> {
    phantom: std::marker::PhantomData<T>,
}
impl<T: Problem> ProblemRunner<T> {
    fn run<P: AsRef<Path> + Copy>(path: P) {
        match T::solve_file(path) {
            Ok(_) => println!("{} passed ✅ ", std::any::type_name::<T>()),
            Err(_) => println!("{} FAILED 🟥", std::any::type_name::<T>()),
        };
    }
}
