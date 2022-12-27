use std::{fs::File, io::BufReader, path::Path};

use crate::day19::Day19;

mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day16_part1;
mod day16_part2;
mod day17;
mod day17_optimized;
mod day17_pretty;
mod day18;
mod day19;
mod day2;
mod day20;
mod day21;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day_template;
mod lkc;

fn main() {
    let mut writer = std::io::Cursor::new(vec![]);
    println!("Hey!");
    let t0 = std::time::Instant::now();
    //Day19::<24, false>::solve_file("in19_small.txt", &mut writer);
    // Day19::<32, true>::solve_file("in19_small.txt", &mut writer);
    //Day19::<24, false>::solve_file("in19.txt", &mut writer);
    //Day19::<32, true>::solve_file("in19.txt", &mut writer);
    Day::<2102>::solve_file("in21.txt", &mut writer);
    println!("Time: {:?}", t0.elapsed());
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum Outcome {
    Loss,
    Draw,
    Win,
}

struct Day<const T: usize> {}

trait Problem {
    fn solve_file<P, W>(path: P, writer: &mut W)
    where
        P: AsRef<Path> + Copy,
        W: std::io::Write,
    {
        let file = File::open(path).expect("File could not be opened.");
        Self::solve_buffer(BufReader::new(file), writer)
    }

    fn solve_buffer<T, W>(reader: BufReader<T>, writer: &mut W)
    where
        T: std::io::Read,
        W: std::io::Write;
}

#[cfg(test)]
mod tests {
    use crate::{
        day11::Day11,
        day14::Day14,
        day15::{Day15, DefinitelyNoBeaconsAtLine, FindTheLoneOutOfRangeTile},
        day17_optimized::Day17Optimized,
        day17_pretty::Day17,
        day20::Day20,
        day3::{Day3CommonItemInCompartments, Day3CommonItemInGroups},
        day4::{Day4, OneFullyInsideAnotherOptimized, OneFullyInsideAnotherSimple, Overlap},
        day5::{CrateMover9000, CrateMover9001, Day5},
        day6::Day6,
        Day, Problem,
    };

    #[test]
    fn test_day() {
        let mut writer = std::io::Cursor::new(vec![]);
        Day::<0>::solve_file("in1.txt", &mut writer);
    }

    #[test]
    #[should_panic]
    fn test_day_badfile() {
        let mut writer = std::io::Cursor::new(vec![]);
        Day::<0>::solve_file("", &mut writer);
    }

    #[test]
    fn day1() {
        let mut writer = std::io::Cursor::new(vec![]);
        Day::<1>::solve_file("in1.txt", &mut writer);
        let output_raw = writer.into_inner();
        let output = std::str::from_utf8(&output_raw).unwrap().trim();
        assert!(output.contains("69528"));
        assert!(output.contains("206152"));
    }

    #[test]
    fn day2() {
        let mut writer = std::io::Cursor::new(vec![]);
        Day::<2>::solve_file("in2.txt", &mut writer);
        let output_raw = writer.into_inner();
        let output = std::str::from_utf8(&output_raw).unwrap().trim();
        assert!(output.contains("13052"));
        assert!(output.contains("13693"));
    }

    #[test]
    fn day3_compartments() {
        let mut writer = std::io::Cursor::new(vec![]);
        Day3CommonItemInCompartments::solve_file("in3.txt", &mut writer);
        let output_raw = writer.into_inner();
        let output = std::str::from_utf8(&output_raw).unwrap().trim();
        assert!(output.contains("7908"));
    }

    #[test]
    fn day3_groups() {
        let mut writer = std::io::Cursor::new(vec![]);
        Day3CommonItemInGroups::solve_file("in3.txt", &mut writer);
        let output_raw = writer.into_inner();
        let output = std::str::from_utf8(&output_raw).unwrap().trim();
        assert!(output.contains("2838"));
    }

    #[test]
    fn day4_fullyinside() {
        let mut writer = std::io::Cursor::new(vec![]);
        Day4::<OneFullyInsideAnotherSimple>::solve_file("in4.txt", &mut writer);
        let output_raw = writer.into_inner();
        let output = std::str::from_utf8(&output_raw).unwrap().trim();
        assert!(output.contains("450"));
    }

    #[test]
    fn day4_fullyinside_optimized() {
        let mut writer = std::io::Cursor::new(vec![]);
        Day4::<OneFullyInsideAnotherOptimized>::solve_file("in4.txt", &mut writer);
        let output_raw = writer.into_inner();
        let output = std::str::from_utf8(&output_raw).unwrap().trim();
        assert!(output.contains("450"));
    }

    #[test]
    fn day4_overlap() {
        let mut writer = std::io::Cursor::new(vec![]);
        Day4::<Overlap>::solve_file("in4.txt", &mut writer);
        let output_raw = writer.into_inner();
        let output = std::str::from_utf8(&output_raw).unwrap().trim();
        assert!(output.contains("837"));
    }

    #[test]
    fn day5_cratemover9000() {
        let mut writer = std::io::Cursor::new(vec![]);
        Day5::<CrateMover9000>::solve_file("in5.txt", &mut writer);
        let output_raw = writer.into_inner();
        let output = std::str::from_utf8(&output_raw).unwrap().trim();
        assert!(output.contains("RLFNRTNFB"));
    }

    #[test]
    fn day5_cratemover9001() {
        let mut writer = std::io::Cursor::new(vec![]);
        Day5::<CrateMover9001>::solve_file("in5.txt", &mut writer);
        let output_raw = writer.into_inner();
        let output = std::str::from_utf8(&output_raw).unwrap().trim();
        assert!(output.contains("MHQTLJRLB"));
    }

    #[test]
    fn day6_signal() {
        let mut writer = std::io::Cursor::new(vec![]);
        Day6::<4>::solve_file("in6.txt", &mut writer);
        let output_raw = writer.into_inner();
        let output = std::str::from_utf8(&output_raw).unwrap().trim();
        assert!(output.contains("1142"));
    }

    #[test]
    fn day6_message() {
        let mut writer = std::io::Cursor::new(vec![]);
        Day6::<14>::solve_file("in6.txt", &mut writer);
        let output_raw = writer.into_inner();
        let output = std::str::from_utf8(&output_raw).unwrap().trim();
        assert!(output.contains("2803"));
    }

    #[test]
    fn day7_small() {
        let mut writer = std::io::Cursor::new(vec![]);
        Day::<7>::solve_file("in7_small.txt", &mut writer);
        let output_raw = writer.into_inner();
        let output = std::str::from_utf8(&output_raw).unwrap().trim();
        assert!(output.contains("95437"));
        assert!(output.contains("24933642"));
    }

    #[test]
    fn day7() {
        let mut writer = std::io::Cursor::new(vec![]);
        Day::<7>::solve_file("in7.txt", &mut writer);
        let output_raw = writer.into_inner();
        let output = std::str::from_utf8(&output_raw).unwrap().trim();
        assert!(output.contains("1477771"));
        assert!(output.contains("3579501"));
    }

    #[test]
    fn day8_small() {
        let mut writer = std::io::Cursor::new(vec![]);
        Day::<8>::solve_file("in8_small.txt", &mut writer);
        let output_raw = writer.into_inner();
        let output = std::str::from_utf8(&output_raw).unwrap().trim();
        assert!(output.contains("21"));
        assert!(output.contains('8'));
    }

    #[test]
    fn day8() {
        let mut writer = std::io::Cursor::new(vec![]);
        Day::<8>::solve_file("in8.txt", &mut writer);
        let output_raw = writer.into_inner();
        let output = std::str::from_utf8(&output_raw).unwrap().trim();
        assert!(output.contains("1705"));
        assert!(output.contains("371200"));
    }

    #[test]
    fn day9() {
        let mut writer = std::io::Cursor::new(vec![]);
        Day::<9>::solve_file("in9.txt", &mut writer);
        let output_raw = writer.into_inner();
        let output = std::str::from_utf8(&output_raw).unwrap().trim();
        assert!(output.contains("6197"));
        assert!(output.contains("2562"));
    }

    #[test]
    fn day10_signal_strength() {
        let mut writer = std::io::Cursor::new(vec![]);
        Day::<101>::solve_file("in10.txt", &mut writer);
        let output_raw = writer.into_inner();
        let output = std::str::from_utf8(&output_raw).unwrap().trim();
        assert!(output.contains("14220"));
    }

    #[test]
    fn day10_crt() {
        let mut writer = std::io::Cursor::new(vec![]);
        Day::<102>::solve_file("in10.txt", &mut writer);
        let output_raw = writer.into_inner();
        let output = std::str::from_utf8(&output_raw).unwrap().trim();
        assert!(output.contains(
            r#"####.###...##..###..#....####.####.#..#.
...#.#..#.#..#.#..#.#....#.......#.#..#.
..#..#..#.#..#.#..#.#....###....#..#..#.
.#...###..####.###..#....#.....#...#..#.
#....#.#..#..#.#.#..#....#....#....#..#.
####.#..#.#..#.#..#.####.#....####..##.."#
        ));
    }

    #[test]
    fn day11() {
        let mut writer = std::io::Cursor::new(vec![]);
        Day11::<3, 20>::solve_file("in11.txt", &mut writer);
        let output_raw = writer.into_inner();
        let output = std::str::from_utf8(&output_raw).unwrap().trim();
        assert!(output.contains("316888"));
    }

    #[test]
    fn day11_big_stress() {
        let mut writer = std::io::Cursor::new(vec![]);
        Day11::<1, 10000>::solve_file("in11.txt", &mut writer);
        let output_raw = writer.into_inner();
        let output = std::str::from_utf8(&output_raw).unwrap().trim();
        assert!(output.contains("35270398814"));
    }

    #[test]
    fn day12() {
        let mut writer = std::io::Cursor::new(vec![]);
        Day::<12>::solve_file("in12.txt", &mut writer);
        let output_raw = writer.into_inner();
        let output = std::str::from_utf8(&output_raw).unwrap().trim();
        assert!(output.contains("412"));
        assert!(output.contains("402"));
    }

    #[test]
    fn day13_already_correct_order() {
        let mut writer = std::io::Cursor::new(vec![]);
        Day::<1301>::solve_file("in13.txt", &mut writer);
        let output_raw = writer.into_inner();
        let output = std::str::from_utf8(&output_raw).unwrap().trim();
        assert!(output.contains("5623"));
    }

    #[test]
    fn day13_delimiters() {
        let mut writer = std::io::Cursor::new(vec![]);
        Day::<1302>::solve_file("in13.txt", &mut writer);
        let output_raw = writer.into_inner();
        let output = std::str::from_utf8(&output_raw).unwrap().trim();
        assert!(output.contains("20570"));
    }

    #[test]
    fn day14_abyss() {
        let mut writer = std::io::Cursor::new(vec![]);
        Day14::<false>::solve_file("in14.txt", &mut writer);
        let output_raw = writer.into_inner();
        let output = std::str::from_utf8(&output_raw).unwrap().trim();
        assert_eq!(output, "768");
    }

    #[test]
    fn day14_floor() {
        let mut writer = std::io::Cursor::new(vec![]);
        Day14::<true>::solve_file("in14.txt", &mut writer);
        let output_raw = writer.into_inner();
        let output = std::str::from_utf8(&output_raw).unwrap().trim();
        assert_eq!(output, "26686");
    }

    #[test]
    fn day15_line2000000() {
        let mut writer = std::io::Cursor::new(vec![]);
        Day15::<DefinitelyNoBeaconsAtLine<2000000>>::solve_file("in15.txt", &mut writer);
        let output_raw = writer.into_inner();
        let output = std::str::from_utf8(&output_raw).unwrap().trim();
        assert!(output.contains("5125700"));
    }

    #[test]
    #[ignore]
    fn day15_out_of_range() {
        let mut writer = std::io::Cursor::new(vec![]);
        Day15::<FindTheLoneOutOfRangeTile<4000000>>::solve_file("in15.txt", &mut writer);
        let output_raw = writer.into_inner();
        let output = std::str::from_utf8(&output_raw).unwrap().trim();
        assert!(output.contains("11379394658764"));
    }

    #[test]
    fn day15_small_line10() {
        let mut writer = std::io::Cursor::new(vec![]);
        Day15::<DefinitelyNoBeaconsAtLine<10>>::solve_file("in15_small.txt", &mut writer);
        let output_raw = writer.into_inner();
        let output = std::str::from_utf8(&output_raw).unwrap().trim();
        assert!(output.contains("26"));
    }

    #[test]
    fn day15_small_out_of_range() {
        let mut writer = std::io::Cursor::new(vec![]);
        Day15::<FindTheLoneOutOfRangeTile<21>>::solve_file("in15_small.txt", &mut writer);
        let output_raw = writer.into_inner();
        let output = std::str::from_utf8(&output_raw).unwrap().trim();
        assert!(output.contains("56000011"));
    }

    #[test]
    fn day16_alone() {
        let mut writer = std::io::Cursor::new(vec![]);
        Day::<1601>::solve_file("in16.txt", &mut writer);
        let output_raw = writer.into_inner();
        let output = std::str::from_utf8(&output_raw).unwrap().trim();
        assert_eq!(output, "2253");
    }

    #[test]
    fn day16_with_elephant() {
        let mut writer = std::io::Cursor::new(vec![]);
        Day::<1602>::solve_file("in16.txt", &mut writer);
        let output_raw = writer.into_inner();
        let output = std::str::from_utf8(&output_raw).unwrap().trim();
        assert_eq!(output, "2838");
    }

    #[test]
    fn day17_tetris_pretty_but_bad() {
        let mut writer = std::io::Cursor::new(vec![]);
        Day17::<2022>::solve_file("in17.txt", &mut writer);
        let output_raw = writer.into_inner();
        let output = std::str::from_utf8(&output_raw).unwrap().trim();
        assert_eq!(output, "3159");
    }

    #[test]
    fn day17_tetris_opt() {
        let mut writer = std::io::Cursor::new(vec![]);
        Day17Optimized::<2022>::solve_file("in17.txt", &mut writer);
        let output_raw = writer.into_inner();
        let output = std::str::from_utf8(&output_raw).unwrap().trim();
        assert_eq!(output, "3159");
    }

    #[test]
    fn day17_tetris_opt_big() {
        let mut writer = std::io::Cursor::new(vec![]);
        Day17Optimized::<1_000_000_000_000>::solve_file("in17.txt", &mut writer);
        let output_raw = writer.into_inner();
        let output = std::str::from_utf8(&output_raw).unwrap().trim();
        assert_eq!(output, "1566272189352");
    }

    #[test]
    fn day18_surface_area() {
        let mut writer = std::io::Cursor::new(vec![]);
        Day::<1801>::solve_file("in18.txt", &mut writer);
        let output_raw = writer.into_inner();
        let output = std::str::from_utf8(&output_raw).unwrap().trim();
        assert_eq!(output, "4332");
    }

    #[test]
    fn day18_outside_surface_area() {
        let mut writer = std::io::Cursor::new(vec![]);
        Day::<1802>::solve_file("in18.txt", &mut writer);
        let output_raw = writer.into_inner();
        let output = std::str::from_utf8(&output_raw).unwrap().trim();
        assert_eq!(output, "2524");
    }

    #[test]
    fn day20_shuffle() {
        let mut writer = std::io::Cursor::new(vec![]);
        Day20::<1, 1>::solve_file("in20.txt", &mut writer);
        let output_raw = writer.into_inner();
        let output = std::str::from_utf8(&output_raw).unwrap().trim();
        assert_eq!(output, "4066");
    }

    #[test]
    fn day20_decryption_key() {
        let mut writer = std::io::Cursor::new(vec![]);
        Day20::<10, 811589153>::solve_file("in20.txt", &mut writer);
        let output_raw = writer.into_inner();
        let output = std::str::from_utf8(&output_raw).unwrap().trim();
        assert_eq!(output, "6704537992933");
    }

    #[test]
    fn day21_eval() {
        let mut writer = std::io::Cursor::new(vec![]);
        Day::<2101>::solve_file("in21.txt", &mut writer);
        let output_raw = writer.into_inner();
        let output = std::str::from_utf8(&output_raw).unwrap().trim();
        assert_eq!(output, "83056452926300");
    }

    #[test]
    fn day21_force_result() {
        let mut writer = std::io::Cursor::new(vec![]);
        Day::<2102>::solve_file("in21.txt", &mut writer);
        let output_raw = writer.into_inner();
        let output = std::str::from_utf8(&output_raw).unwrap().trim();
        assert_eq!(output, "3469704905529");
    }
}
