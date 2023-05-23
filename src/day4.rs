use std::{io::prelude::*, io::BufReader, marker::PhantomData};

use lk_math::interval::{Interval, IntervalExt};

use crate::Problem;

trait IntervalRelation {
    fn test(a0: i32, a1: i32, b0: i32, b1: i32) -> bool;
    fn name() -> &'static str;
}

pub struct OneFullyInsideAnother;
pub struct Overlap;

impl IntervalRelation for OneFullyInsideAnother {
    fn test(a0: i32, a1: i32, b0: i32, b1: i32) -> bool {
        let a = a0..a1 + 1;
        let b = b0..b1 + 1;
        assert_eq!(
            a.dominates_or_is_dominated_by(&b),
            (a0 >= b0 && a1 <= b1) || (b0 >= a0 && b1 <= a1)
        );
        a.dominates_or_is_dominated_by(&b)
        // (a0 >= b0 && a1 <= b1) || (b0 >= a0 && b1 <= a1)
    }

    fn name() -> &'static str {
        "One fully overlaps the other"
    }
}
impl IntervalRelation for Overlap {
    fn test(a0: i32, a1: i32, b0: i32, b1: i32) -> bool {
        let a = a0..a1 + 1;
        let b = b0..b1 + 1;
        assert_eq!(a.overlaps(&b), a1 >= b0 && a0 <= b1);
        a.overlaps(&b)
        // a1 >= b0 && a0 <= b1
    }

    fn name() -> &'static str {
        "Any overlap"
    }
}

pub struct Day4<T> {
    phantom: PhantomData<T>,
}

impl<T> Problem for Day4<T>
where
    T: IntervalRelation,
{
    fn solve_buffer<U, W>(reader: BufReader<U>, writer: &mut W)
    where
        U: std::io::Read,
        W: std::io::Write,
    {
        let tests_passed: Result<i32, ()> = reader
            .lines()
            .map(|line| -> Result<i32, ()> {
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
                            let a0 = a0.parse::<i32>().unwrap();
                            let a1 = a1.parse::<i32>().unwrap();
                            let b0 = b0.parse::<i32>().unwrap();
                            let b1 = b1.parse::<i32>().unwrap();

                            return Ok(match T::test(a0, a1, b0, b1) {
                                true => 1,
                                false => 0,
                            });
                        }
                    }
                }
                Err(())
            })
            .sum();

        if let Ok(tests_passed) = tests_passed {
            writeln!(writer, "{}: {}", T::name(), tests_passed).unwrap()
        }
    }
}
