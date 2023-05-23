use std::{collections::HashMap, io::prelude::*, io::BufReader};

use crate::{Day, Problem};
use lk_math::expr::Expr;

impl Problem for Day<2101> {
    fn solve_buffer<T, W>(reader: BufReader<T>, writer: &mut W)
    where
        T: std::io::Read,
        W: std::io::Write,
    {
        let mut monkeys: HashMap<String, _> = HashMap::new();

        for line in reader.lines().map(|x| x.unwrap()) {
            let (a, b) = line.split_once(':').unwrap();
            let expr = b.parse::<Expr<i64>>().unwrap();
            monkeys.insert(a.into(), expr);
        }

        let result = monkeys.get("root").unwrap().eval(&monkeys).unwrap();
        println!("Result {}", result);
        writeln!(writer, "{}", result).unwrap();
    }
}

impl Problem for Day<2102> {
    fn solve_buffer<T, W>(reader: BufReader<T>, writer: &mut W)
    where
        T: std::io::Read,
        W: std::io::Write,
    {
        let mut monkeys: HashMap<String, _> = HashMap::new();

        for line in reader.lines().map(|x| x.unwrap()) {
            let (a, b) = line.split_once(':').unwrap();

            let expr = match a {
                "root" => b.replace(['+', '-', '*', '/'], "=").parse().unwrap(),
                "humn" => Expr::Free,
                _ => b.parse().unwrap(),
            };

            monkeys.insert(a.into(), expr);
        }

        let forced: HashMap<String, i64> =
            monkeys.get("root").unwrap().solve(true.into(), &monkeys);
        println!("Forced: {:?}", forced);

        writeln!(writer, "{}", forced.get("humn").unwrap()).unwrap();
    }
}
