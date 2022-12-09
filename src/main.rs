use std::path::Path;

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day_template;

fn main() {
    ProblemRunner::<Day<0>>::run("in1.txt");
    ProblemRunner::<Day<1>>::run("in1.txt");
    ProblemRunner::<Day<2>>::run("in2.txt");
    ProblemRunner::<Day<3>>::run("in3.txt");
    ProblemRunner::<Day<4>>::run("in4.txt");
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum Outcome {
    Loss,
    Draw,
    Win,
}

struct Day<const T: usize> {}

trait Problem {
    fn solve_file<P: AsRef<Path> + Copy>(path: P) -> Result<(), ()>;
}

struct ProblemRunner<T> {
    phantom: std::marker::PhantomData<T>,
}
impl<T: Problem> ProblemRunner<T> {
    fn run<P: AsRef<Path> + Copy>(path: P) {
        match T::solve_file(path) {
            Ok(_) => println!("{} passed :)", std::any::type_name::<T>()),
            Err(_) => println!("{} FAILED!!!!", std::any::type_name::<T>()),
        };
    }
}
