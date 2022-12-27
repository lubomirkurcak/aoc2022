use std::{collections::HashMap, io::prelude::*, io::BufReader};

use crate::{lkc::expr::Expression, Day, Problem};

impl Problem for Day<2101> {
    fn solve_buffer<T, W>(reader: BufReader<T>, writer: &mut W)
    where
        T: std::io::Read,
        W: std::io::Write,
    {
        let mut monkeys = HashMap::new();

        for line in reader.lines().map(|x| x.unwrap()) {
            let (a, b) = line.split_once(':').unwrap();
            let expr = Expression::<i64>::from_str(b);
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
                "root" => Expression::from_str(&b.replace(['+', '-', '*', '/'], "=")),
                "humn" => Expression::Free,
                _ => Expression::<i64>::from_str(b),
            };

            monkeys.insert(a.into(), expr);
        }

        let forced = monkeys
            .get("root")
            .unwrap()
            .force_result(true.into(), &monkeys);
        println!("Forced: {:?}", forced);

        writeln!(writer, "{}", forced.get("humn").unwrap()).unwrap();
    }
}
