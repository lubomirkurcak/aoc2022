use std::{collections::HashMap, io::prelude::*, io::BufReader};

use crate::lkc::math::*;
use crate::Problem;

type MonkeyId = usize;
type WorryLevel = i64;

struct Monkey {
    items: Vec<WorryLevel>,
    operation: Box<dyn Fn(WorryLevel) -> WorryLevel>,
    test: Box<dyn Fn(WorryLevel) -> bool>,
    target_if_true: MonkeyId,
    target_if_false: MonkeyId,
    items_inspected: usize,
    division_test_value: WorryLevel,
}

enum Expression {
    Add(Box<Expression>, Box<Expression>),
    Mul(Box<Expression>, Box<Expression>),
    Old,
    Const(WorryLevel),
}

impl Expression {
    fn eval(&self, old: WorryLevel) -> WorryLevel {
        match self {
            Expression::Add(a, b) => a.eval(old) + b.eval(old),
            Expression::Mul(a, b) => a.eval(old) * b.eval(old),
            Expression::Old => old,
            Expression::Const(val) => *val,
        }
    }

    fn from_str(s: &str) -> Self {
        let s = s.trim();

        if let Some(i) = s.find('+') {
            let split = s.split_at(i);
            return Self::Add(
                Box::new(Self::from_str(split.0)),
                Box::new(Self::from_str(&split.1[1..])),
            );
        }

        if let Some(i) = s.find('*') {
            let split = s.split_at(i);
            return Self::Mul(
                Box::new(Self::from_str(split.0)),
                Box::new(Self::from_str(&split.1[1..])),
            );
        }

        if s == "old" {
            return Self::Old;
        }

        if let Ok(val) = s.parse() {
            return Self::Const(val);
        }

        println!("Unknown expression: '{}'", s);

        panic!();
    }
}

impl Monkey {
    fn from_buffer<T>(reader: BufReader<T>) -> HashMap<MonkeyId, Self>
    where
        T: std::io::Read,
    {
        let mut lines = reader.lines().map(|x| x.unwrap());

        let mut results = HashMap::new();

        while let Some(line) = lines.next() {
            let id = line.split("Monkey").collect::<Vec<&str>>()[1];
            let id = id[..id.len() - 1].trim().parse().unwrap();

            let line = lines.next().unwrap();
            let items = line.split("Starting items:").collect::<Vec<&str>>()[1];
            let items = items.trim().split(',').collect::<Vec<&str>>();
            let items = items.iter().map(|x| x.trim().parse().unwrap()).collect();

            let line = lines.next().unwrap();
            let operation = line.split("Operation:").collect::<Vec<&str>>()[1];
            let operation = operation.trim().split('=').collect::<Vec<&str>>()[1];
            let expression = Expression::from_str(operation);
            let operation = Box::new(move |old| expression.eval(old));

            let line = lines.next().unwrap();
            let divisible_by: WorryLevel = line.split("Test: divisible by").collect::<Vec<&str>>()
                [1]
            .trim()
            .parse()
            .unwrap();
            let test = Box::new(move |x| x % divisible_by == 0);

            let line = lines.next().unwrap();
            let target_if_true = line
                .split("If true: throw to monkey")
                .collect::<Vec<&str>>()[1]
                .trim()
                .parse()
                .unwrap();
            let line = lines.next().unwrap();

            let target_if_false = line
                .split("If false: throw to monkey")
                .collect::<Vec<&str>>()[1]
                .trim()
                .parse()
                .unwrap();

            results.insert(
                id,
                Self {
                    items,
                    operation,
                    test,
                    target_if_true,
                    target_if_false,
                    items_inspected: 0,
                    division_test_value: divisible_by,
                },
            );

            if let Some(line) = lines.next() {
                assert!(line.is_empty())
            }
        }

        results
    }
}

pub struct Day11<const D: WorryLevel, const R: usize> {}
impl<const D: WorryLevel, const R: usize> Problem for Day11<D, R> {
    fn solve_buffer<T, W>(reader: BufReader<T>, writer: &mut W)
    where
        T: std::io::Read,
        W: std::io::Write,
    {
        let mut monkeys = Monkey::from_buffer(reader);

        let mut modulo = 1;
        for x in monkeys.values() {
            modulo = WorryLevel::lcm(modulo, x.division_test_value);
        }
        println!("Divisor LCM: {}", modulo);
        let straight_product: WorryLevel =
            monkeys.values().map(|x| x.division_test_value).product();
        println!("Straight product: {}", straight_product);

        let mut monkey_keys: Vec<_> = monkeys.keys().copied().collect();
        monkey_keys.sort();
        let monkey_keys = monkey_keys;
        for _round in 0..R {
            for monkey_id in monkey_keys.iter() {
                let monkey = monkeys.get_mut(monkey_id).unwrap();
                let id_if_true = monkey.target_if_true;
                let id_if_false = monkey.target_if_false;

                monkey.items_inspected += monkey.items.len();
                let (true_items, false_items): (Vec<_>, Vec<_>) = monkey
                    .items
                    .drain(..)
                    .map(|x| ((monkey.operation)(x) / D) % modulo)
                    .partition(|x| (monkey.test)(*x));

                let true_target = monkeys.get_mut(&id_if_true).unwrap();
                true_target.items.extend(true_items);

                let false_target = monkeys.get_mut(&id_if_false).unwrap();
                false_target.items.extend(false_items);
            }
        }

        let mut inspected: Vec<_> = monkeys.values().map(|x| x.items_inspected).collect();
        inspected.sort();
        inspected.reverse();
        let monkey_business: usize = inspected.iter().take(2).product();
        writeln!(writer, "Monkey business: {}", monkey_business).unwrap();
    }
}
