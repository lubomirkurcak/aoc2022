use std::{fs::File, io::prelude::*, io::BufReader, path::Path};

use crate::{Day, Problem, Outcome};

impl Problem for Day<2> {
    fn solve_file<P: AsRef<Path>>(path: P) -> Result<(), ()> {
        let file = match File::open(path) {
            Ok(it) => it,
            Err(_) => return Err(()),
        };
        let reader = BufReader::new(file);

        #[derive(Clone, Copy, PartialEq, Eq)]
        enum Hands {
            Rock,
            Paper,
            Scissors,
        }

        impl PartialOrd for Hands {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                match self {
                    Hands::Rock => match other {
                        Hands::Rock => Some(std::cmp::Ordering::Equal),
                        Hands::Paper => Some(std::cmp::Ordering::Less),
                        Hands::Scissors => Some(std::cmp::Ordering::Greater),
                    },
                    Hands::Paper => match other {
                        Hands::Rock => Some(std::cmp::Ordering::Greater),
                        Hands::Paper => Some(std::cmp::Ordering::Equal),
                        Hands::Scissors => Some(std::cmp::Ordering::Less),
                    },
                    Hands::Scissors => match other {
                        Hands::Rock => Some(std::cmp::Ordering::Less),
                        Hands::Paper => Some(std::cmp::Ordering::Greater),
                        Hands::Scissors => Some(std::cmp::Ordering::Equal),
                    },
                }
            }
        }

        assert!(Hands::Paper < Hands::Scissors);
        assert!(Hands::Scissors < Hands::Rock);
        assert!(Hands::Rock < Hands::Paper);

        fn opp_hand(x: &str) -> Hands {
            match x {
                "A" => Hands::Rock,
                "B" => Hands::Paper,
                _ => Hands::Scissors,
            }
        }
        fn my_hand(x: &str) -> Hands {
            match x {
                "X" => Hands::Rock,
                "Y" => Hands::Paper,
                _ => Hands::Scissors,
            }
        }
        fn hand_needed_for_outcome(opp: Hands, outcome: Outcome) -> Hands {
            match opp {
                Hands::Rock => match outcome {
                    Outcome::Loss => Hands::Scissors,
                    Outcome::Draw => Hands::Rock,
                    Outcome::Win => Hands::Paper,
                },
                Hands::Paper => match outcome {
                    Outcome::Loss => Hands::Rock,
                    Outcome::Draw => Hands::Paper,
                    Outcome::Win => Hands::Scissors,
                },
                Hands::Scissors => match outcome {
                    Outcome::Loss => Hands::Paper,
                    Outcome::Draw => Hands::Scissors,
                    Outcome::Win => Hands::Rock,
                },
            }
        }

        let hand_score = |hand: Hands| match hand {
            Hands::Rock => 1,
            Hands::Paper => 2,
            Hands::Scissors => 3,
        };

        let outcome_score = |hand1: Hands, hand2: Hands| match hand1.partial_cmp(&hand2) {
            Some(order) => match order {
                std::cmp::Ordering::Less => 0,
                std::cmp::Ordering::Equal => 3,
                std::cmp::Ordering::Greater => 6,
            },
            None => panic!(),
        };

        let total_score: (i32, i32) = reader
            .lines()
            .map(|line| {
                let line = match line {
                    Ok(line) => line,
                    Err(_) => return Err(()),
                };

                let vec: Vec<&str> = line.split_ascii_whitespace().collect();
                match vec[..] {
                    [first, second] => {
                        let opp = opp_hand(first);
                        let me = my_hand(second);

                        let needed_outcome = match second {
                            "X" => Outcome::Loss,
                            "Y" => Outcome::Draw,
                            _ => Outcome::Win,
                        };
                        let needed_hand = hand_needed_for_outcome(opp, needed_outcome);

                        let score1 = hand_score(me) + outcome_score(me, opp);
                        let score2 = hand_score(needed_hand) + outcome_score(needed_hand, opp);
                        return Ok((score1, score2));
                    }
                    _ => Err(()),
                }
            })
            .map(|x| match x {
                Ok(score) => score,
                Err(_) => (0, 0),
            })
            .fold((0, 0), |acc, x| {
                let (acc0, acc1) = acc;
                let (x0, x1) = x;
                return (acc0 + x0, acc1 + x1);
            });

        let (total_score1, total_score2) = total_score;
        println!("Total score 1: {}", total_score1);
        println!("Total score 2: {}", total_score2);

        Ok(())
    }
}
