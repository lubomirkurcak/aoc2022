use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    hash::Hash,
    io::prelude::*,
    io::BufReader,
};

use crate::{Day, Problem};

fn visible_from_start<I>(array: I) -> HashSet<usize>
where
    I: Iterator,
    I::Item: PartialOrd + Copy,
{
    let mut results = HashSet::new();

    let mut tallest_prev = None;

    for (idx, ch) in array.enumerate() {
        match tallest_prev {
            Some(value) => {
                if ch > value {
                    tallest_prev = Some(ch);
                    results.insert(idx);
                }
            }
            None => {
                tallest_prev = Some(ch);
                results.insert(idx);
            }
        }
    }

    results
}

fn visible_from_either_end(v: &[char]) -> HashSet<usize> {
    let mut visible = visible_from_start(v.iter());
    let from_right = visible_from_start(v.iter().rev())
        .iter()
        .map(|i| v.len() - i - 1)
        .collect::<HashSet<usize>>();

    visible.extend(from_right);
    visible
}

fn map_x<X, Y>(const_x: X, items: HashSet<Y>) -> HashSet<(X, Y)>
where
    X: Copy + Eq + Hash,
    Y: Copy + Eq + Hash,
{
    items.iter().map(|y: &Y| (const_x, *y)).collect()
}
fn map_y<X, Y>(items: HashSet<X>, const_y: Y) -> HashSet<(X, Y)>
where
    X: Copy + Eq + Hash,
    Y: Copy + Eq + Hash,
{
    items.iter().map(|x: &X| (*x, const_y)).collect()
}

fn count_contiguous_less_than_values<I>(array: I, threshold: I::Item) -> usize
where
    I: Iterator,
    I::Item: PartialOrd + Copy,
{
    let mut result = 0;
    for val in array {
        result += 1;
        if val < threshold {
        } else {
            break;
        }
    }
    result
}

struct Array2D {
    width: usize,
    height: usize,

    rows: HashMap<usize, Vec<char>>,
    columns: HashMap<usize, Vec<char>>,
}

impl Display for Array2D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in (0..self.height).rev() {
            writeln!(
                f,
                "{}",
                self.rows.get(&y).unwrap().iter().collect::<String>()
            )?;
        }
        write!(f, "")
    }
}

impl Array2D {
    fn new() -> Self {
        Self {
            width: 0,
            height: 0,
            rows: HashMap::new(),
            columns: HashMap::new(),
        }
    }

    fn from_buffer<T>(reader: BufReader<T>) -> Result<Self, ()>
    where
        T: std::io::Read,
    {
        let mut array = Self::new();
        for (line_index, line) in reader.lines().enumerate() {
            let line = match line {
                Ok(line) => line,
                Err(_) => return Err(()),
            };

            array.width = std::cmp::max(array.width, line.len());
            array.height = line_index + 1;

            let row = line.chars().collect::<Vec<char>>();
            for (col, x) in row.iter().enumerate() {
                array.columns.entry(col).or_default().push(*x);
            }
            array.rows.insert(line_index, row);
        }

        Ok(array)
    }

    fn scenic_score(&self, x: usize, y: usize) -> usize {
        let row = self.rows.get(&y).unwrap();
        let col = self.columns.get(&x).unwrap();

        let current = &row[x];
        let right = count_contiguous_less_than_values(row[x + 1..].iter(), current);
        let left = count_contiguous_less_than_values(row[..x].iter().rev(), current);
        let up = count_contiguous_less_than_values(col[y + 1..].iter(), current);
        let down = count_contiguous_less_than_values(col[..y].iter().rev(), current);

        right * up * left * down
    }
}

impl Problem for Day<8> {
    fn solve_buffer<T, W>(reader: BufReader<T>, writer: &mut W)
    where
        T: std::io::Read,
        W: std::io::Write,
    {
        let mut all_visible_trees = HashSet::new();

        let array = Array2D::from_buffer(reader).unwrap();

        for (col_id, row) in array.rows.iter() {
            let visible1d = visible_from_either_end(row);
            let visible2d = map_y(visible1d, *col_id);
            all_visible_trees.extend(visible2d);
        }

        for (row_id, col) in array.columns.iter() {
            let visible1d = visible_from_either_end(col);
            let visible2d = map_x(*row_id, visible1d);
            all_visible_trees.extend(visible2d);
        }

        // writeln!("{}", array);

        writeln!(writer, "Visible trees: {}", all_visible_trees.len()).unwrap();

        let scenic_scores = (0..array.height)
            .flat_map(|y| {
                (0..array.width)
                    .map(|x| (x, y, array.scenic_score(x, y)))
                    .collect::<Vec<(usize, usize, usize)>>()
            })
            .filter(|(_, _, score)| *score != 0)
            .collect::<Vec<(usize, usize, usize)>>();

        // writeln!("{:?}", scenic_scores);

        let scenic = scenic_scores
            .iter()
            .max_by(|(_, _, a), (_, _, b)| a.cmp(b))
            .unwrap();
        let (x, y, score) = scenic;

        writeln!(writer, "Max scenic score: {}, at ({},{})", score, x, y).unwrap();
    }
}
