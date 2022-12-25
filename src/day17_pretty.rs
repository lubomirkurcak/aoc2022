use std::io::{BufRead, BufReader};

use crate::{
    day17::Rock,
    lkc::{
        array2d::Array2d,
        cli::Progress,
        explore::{Exploration, ExploreSignals},
        line::LineV2i32,
        vector::V2,
    },
    Problem,
};

pub struct Day17<const C: usize>;

impl<const C: usize> Problem for Day17<C> {
    fn solve_buffer<T, W>(reader: BufReader<T>, writer: &mut W)
    where
        T: std::io::Read,
        W: std::io::Write,
    {
        let check = |map: &Array2d<char>, blueprint: &Vec<_>, p| {
            blueprint
                .iter()
                .map(|line: &LineV2i32| {
                    map.iter_values_in_line(line.start + p, line.end + p)
                        .all(|c| c == &'.')
                })
                .all(|x| x)
        };

        let draw = |map: &mut Array2d<char>, blueprint: &Vec<LineV2i32>, p| {
            blueprint.iter().for_each(|line| {
                map.draw_line(line.start + p, line.end + p, '#');
            })
        };

        let wind = reader
            .lines()
            .map(|x| x.unwrap())
            .flat_map(|x| x.chars().collect::<Vec<_>>())
            .map(|x| match x {
                '>' => 1,
                '<' => -1,
                _ => panic!(),
            })
            .collect::<Vec<_>>();
        let wind_count = wind.len();
        println!("Wind length: {}", wind_count);

        let mut wind = wind.iter().cycle();

        let mut map = Array2d::new(7, 100, '.');

        let mut progress = Progress::new(C);
        let mut first_free_row_absolute = 0u64;
        let mut first_free_row = 0;
        let mut first_non_full_row = 0;
        for (iteration, rock_type) in (0..5).cycle().enumerate().take(C) {
            let mut p = V2::from_xy(2, first_free_row + 3);
            let blueprint = Rock::construct(rock_type);
            let width = Rock::width(rock_type);
            let height = Rock::height(rock_type);

            progress.progress(iteration);

            assert!(check(&map, &blueprint, p));

            loop {
                let x = (p.x() + *wind.next().unwrap()).clamp(0, 7 - width);
                if p.x() != x && check(&map, &blueprint, V2::from_xy(x, p.y())) {
                    p.values[0] = x;
                }

                if p.y() > 0 && check(&map, &blueprint, V2::from_xy(p.x(), p.y() - 1)) {
                    p.values[1] -= 1;
                } else {
                    draw(&mut map, &blueprint, p);
                    first_free_row = std::cmp::max(first_free_row, p.y() + height);

                    loop {
                        let a = map
                            .line_iter(
                                V2::from_xy(0, first_non_full_row),
                                V2::from_xy(6, first_non_full_row),
                            )
                            .find(|&x| map.get(x).unwrap() == &'.');

                        if let Some(a) = a {
                            let mut failed = false;
                            let mut exp = Exploration::new(map);
                            exp.explore(
                                a,
                                |x, map| {
                                    if x.y() >= first_free_row {
                                        failed = true;
                                        return ExploreSignals::ReachedGoal;
                                    }
                                    map.set(*x, '@');
                                    // println!("{}", x);
                                    ExploreSignals::Explore
                                },
                                |x, map| x.y() <= first_free_row && map.get(*x).unwrap() == &'.',
                            );
                            // NOTE(lubo): Exploration returns the map
                            map = exp.context;

                            // println!("{}", map);

                            if failed {
                                map.replace_all(&'@', &'.');
                                break;
                            } else {
                                map.replace_all(&'@', &'X');
                            }
                        } else {
                            first_non_full_row += 1;
                        }
                    }

                    map.shift_n_rows_down(first_non_full_row.try_into().unwrap(), '.');
                    first_free_row_absolute +=
                        <i32 as std::convert::TryInto<u64>>::try_into(first_non_full_row).unwrap();
                    first_free_row -= first_non_full_row;
                    first_non_full_row = 0;

                    break;
                }
            }

            if iteration % wind_count == 0 {
                println!("{}", map);
            }
        }

        first_free_row_absolute +=
            <i32 as std::convert::TryInto<u64>>::try_into(first_free_row).unwrap();

        println!("{}", first_free_row_absolute);

        writeln!(writer, "{}", first_free_row_absolute).unwrap();
    }
}
