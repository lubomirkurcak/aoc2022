use std::{cmp::Ordering, io::prelude::*, io::BufReader};

use crate::{Day, Problem};

#[derive(Debug, Clone, Eq)]
enum Token {
    Value(i32),
    List(Vec<Token>),
}

trait InsertIfSome<T> {
    fn insert_if_some(&mut self, element: Option<T>);
}

impl<T> InsertIfSome<T> for Vec<T> {
    fn insert_if_some(&mut self, maybe_element: Option<T>) {
        if let Some(element) = maybe_element {
            self.push(element)
        }
    }
}

impl Token {
    fn parse_value(s: &str) -> Self {
        Self::Value(s.parse().unwrap())
    }

    fn make_list(&self) -> Self {
        match self {
            Token::Value(_) => Self::List(vec![self.clone()]),
            Token::List(_) => self.clone(),
        }
    }

    fn parse_list(s: &str) -> Self {
        assert!(s.starts_with('['));
        assert!(s.ends_with(']'));
        let mut depth = 0;
        let mut last_delim = None;
        let mut elements = vec![];
        for (idx, ch) in s.chars().enumerate() {
            match ch {
                '[' => depth += 1,
                ']' => depth -= 1,
                ',' => {
                    assert!(depth >= 1);
                    if depth == 1 {
                        match last_delim {
                            Some(last) => {
                                elements.insert_if_some(Token::from_str(&s[last + 1..idx]));
                            }
                            None => {
                                elements.insert_if_some(Token::from_str(&s[1..idx]));
                            }
                        }

                        last_delim = Some(idx);
                    }
                }
                _ => (),
            }

            assert!(depth >= 0);

            if depth == 0 {
                assert_eq!(idx, s.len() - 1);

                match last_delim {
                    Some(last) => {
                        elements.insert_if_some(Token::from_str(&s[last + 1..idx]));
                    }
                    None => {
                        elements.insert_if_some(Token::from_str(&s[1..idx]));
                    }
                }
            }
        }
        assert_eq!(depth, 0);
        Self::List(elements)
    }

    fn from_str(s: &str) -> Option<Self> {
        if let Some(ch) = s.chars().next() {
            match ch {
                '[' => Some(Self::parse_list(s)),
                '0'..='9' => Some(Self::parse_value(s)),
                _ => panic!(),
            }
        } else {
            None
        }
    }
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        match self.partial_cmp(other) {
            Some(ordering) => match ordering {
                std::cmp::Ordering::Less => false,
                std::cmp::Ordering::Equal => true,
                std::cmp::Ordering::Greater => false,
            },
            None => false,
        }
    }
}

impl Ord for Token {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialOrd for Token {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self {
            Token::Value(val) => match other {
                Token::Value(oval) => val.partial_cmp(oval),
                Token::List(_) => self.make_list().partial_cmp(other),
            },
            Token::List(list) => match other {
                Token::Value(_) => self.partial_cmp(&other.make_list()),
                Token::List(olist) => {
                    let mut a = list.iter();
                    let mut b = olist.iter();

                    let mut list_ordering = Ordering::Equal;
                    let mut looping = true;
                    while looping {
                        looping = false;
                        let c = a.next();
                        let d = b.next();

                        if let Some(c) = c {
                            if let Some(d) = d {
                                match c.partial_cmp(d).unwrap() {
                                    std::cmp::Ordering::Equal => looping = true,
                                    a => list_ordering = a,
                                }
                            } else {
                                list_ordering = Ordering::Greater;
                            }
                        } else if d.is_some() {
                            list_ordering = Ordering::Less;
                        }
                    }

                    Some(list_ordering)
                }
            },
        }
    }
}

impl Problem for Day<1301> {
    fn solve_buffer<T, W>(reader: BufReader<T>, writer: &mut W)
    where
        T: std::io::Read,
        W: std::io::Write,
    {
        let mut packet_pair_index = 1;
        let mut index_sum = 0;

        let mut lines = reader.lines().map(|x| x.unwrap());

        while let Some(left) = lines.next() {
            let right = lines.next().unwrap();
            let _empty = lines.next();

            let l = Token::from_str(&left);
            let r = Token::from_str(&right);

            if l < r {
                // println!( "Wrong order at packet pair with index {}: A < B\nA: {}\nB: {}", packet_pair_index, left, right);
                index_sum += packet_pair_index;
            }

            packet_pair_index += 1;
        }

        writeln!(writer, "{}", index_sum).unwrap();
    }
}

impl Problem for Day<1302> {
    fn solve_buffer<T, W>(reader: BufReader<T>, writer: &mut W)
    where
        T: std::io::Read,
        W: std::io::Write,
    {
        let mut lines = reader
            .lines()
            .map(|x| x.unwrap())
            .filter_map(|x| match !x.is_empty() {
                true => Some(x),
                false => None,
            })
            .map(|x| Token::from_str(&x).unwrap())
            .collect::<Vec<_>>();

        let div2 = Token::from_str("[[2]]").unwrap();
        let div6 = Token::from_str("[[6]]").unwrap();

        let mut div2_idx = None;
        let mut div6_idx = None;
        lines.push(div2.clone());
        lines.push(div6.clone());
        lines.sort();
        for (idx, packet) in lines.iter().enumerate() {
            if packet.clone() == div2 {
                div2_idx = Some(idx + 1);
            }
        }
        for (idx, packet) in lines.iter().enumerate() {
            if packet.clone() == div6 {
                div6_idx = Some(idx + 1);
            }
        }
        let div2_idx = div2_idx.unwrap();
        let div6_idx = div6_idx.unwrap();
        writeln!(writer, "{}", div2_idx * div6_idx).unwrap();
    }
}
