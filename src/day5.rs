use std::{collections::HashMap, io::prelude::*, io::BufReader, marker::PhantomData};

use crate::Problem;

trait CrateMover {
    fn move_crates(stacks: &mut HashMap<usize, Vec<char>>, count: usize, from: usize, to: usize);
}

pub struct CrateMover9000;
pub struct CrateMover9001;

impl CrateMover for CrateMover9000 {
    fn move_crates(stacks: &mut HashMap<usize, Vec<char>>, count: usize, from: usize, to: usize) {
        for _ in 0..count {
            if let Some(source) = stacks.get_mut(&from) {
                if let Some(char) = source.pop() {
                    if let Some(target) = stacks.get_mut(&to) {
                        target.push(char);
                    }
                }
            }
        }
    }
}

impl CrateMover for CrateMover9001 {
    fn move_crates(stacks: &mut HashMap<usize, Vec<char>>, count: usize, from: usize, to: usize) {
        if let Some(source) = stacks.get_mut(&from) {
            let new_source_len = source.len() - count;
            let mut lifted = source[new_source_len..].to_owned();
            source.truncate(new_source_len);
            if let Some(target) = stacks.get_mut(&to) {
                target.append(&mut lifted);
            }
        }
    }
}

enum Stages {
    ReadingInitialState,
    MovingCrates,
}

pub struct Day5<T> {
    phantom: PhantomData<T>,
}

impl<T> Problem for Day5<T>
where
    T: CrateMover,
{
    fn solve_buffer<U, W>(reader: BufReader<U>, writer: &mut W)
    where
        U: std::io::Read,
        W: std::io::Write,
    {
        let mut stage = Stages::ReadingInitialState;

        let mut stacks: HashMap<usize, Vec<char>> = HashMap::new();
        for line in reader.lines().map(|x| x.unwrap()) {
            stage = match stage {
                Stages::ReadingInitialState => {
                    if line.contains('[') {
                        let mut chars = line.chars();
                        chars.next();
                        for (index, char) in chars.step_by(4).enumerate() {
                            // NOTE(lubo): Columns/stacks in the puzzle are indexed from 1
                            let index = index + 1;

                            if char != ' ' {
                                stacks.entry(index).or_default().push(char);
                            }
                        }
                        Stages::ReadingInitialState
                    } else if line.trim().is_empty() {
                        for stack in stacks.values_mut() {
                            stack.reverse();
                        }
                        Stages::MovingCrates
                    } else {
                        Stages::ReadingInitialState
                    }
                }
                Stages::MovingCrates => {
                    if let [_, a] = line.split("move").collect::<Vec<&str>>()[..] {
                        if let [count, a] = a.split("from").collect::<Vec<&str>>()[..] {
                            if let [from, to] = a.split("to").collect::<Vec<&str>>()[..] {
                                if let Ok(count) = count.trim().parse::<usize>() {
                                    if let Ok(from) = from.trim().parse::<usize>() {
                                        if let Ok(to) = to.trim().parse::<usize>() {
                                            T::move_crates(&mut stacks, count, from, to);
                                        }
                                    }
                                }
                            }
                        }
                    }
                    Stages::MovingCrates
                }
            };
        }

        let mut keys = stacks.keys().cloned().collect::<Vec<usize>>();
        keys.sort();

        let top_elements: String = keys
            .iter()
            .map(|key| stacks.get(key).unwrap().last().unwrap())
            .collect();

        //let top_elements: String = stacks.values().map(|stack| stack.last().unwrap()).collect();
        writeln!(
            writer,
            "Using {}, Top items from each stack: {}",
            std::any::type_name::<T>(),
            top_elements
        )
        .unwrap();
    }
}
