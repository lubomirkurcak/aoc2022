use std::{cmp::max, collections::HashMap, io::prelude::*, io::BufReader};

use crate::{Day, Problem};

pub struct Day6<const T: usize>;

// NOTE(lubo): This was found to be performant enough... :(
fn explicit_compare<const C: usize>(chars: Vec<char>) -> Option<usize> {
    'outer: for (index, window) in chars.windows(C).enumerate() {
        for a in window.iter() {
            for b in window[1..].iter() {
                if !std::ptr::eq(a, b) && a == b {
                    continue 'outer;
                }
            }
        }

        return Some(index + C);
    }
    None
}

fn skip_as_far_as_possible_upon_finding_a_duplicate<const C: usize>(
    chars: Vec<char>,
) -> Option<usize> {
    let mut signal_offset = 0;
    let window = &chars[signal_offset..signal_offset + C];

    for (index, window) in chars.windows(4).enumerate() {
        let index_of_last = C - 1;
        let last = window[index_of_last];
        for (id, &ch) in window[..C - 1].iter().enumerate().rev() {
            if last == ch {
                let match_relative_difference = index_of_last - id;
                let skip = C - match_relative_difference;
                println!(
                    "Match found at relative difference: {}, Safe to skip {}",
                    match_relative_difference, skip
                )
            }
        }
    }

    None
}

//fn hashmap(){
// let mut latest_occurences = HashMap::new();
// for (index, &char) in chars.iter().enumerate() {
//     let latest = latest_occurences.entry(char).or_default();
//     *latest = index;
// }
//}

impl<const C: usize> Problem for Day6<C> {
    fn solve_buffer<T>(reader: BufReader<T>) -> Result<(), ()>
    where
        T: std::io::Read,
    {
        let mut signal_starts_at = 0;
        let mut lines = reader.lines();
        if let Some(Ok(first)) = lines.next() {
            println!("Line length: {}", first.len());
            let chars = first.chars().collect::<Vec<char>>();
            signal_starts_at = explicit_compare::<C>(chars).unwrap();
        }

        println!("Signal starts at: {}", signal_starts_at);

        Ok(())
    }
}
