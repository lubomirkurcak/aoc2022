use std::{fs::File, io::BufReader, path::Path};

pub mod day1;
pub mod day10;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;
pub mod day_template;

fn main() {
    println!("Hey!");
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
        let output = std::str::from_utf8(&output_raw).unwrap();
        assert!(output.contains("69528"));
        assert!(output.contains("206152"));
    }

    #[test]
    fn day2() {
        let mut writer = std::io::Cursor::new(vec![]);
        Day::<2>::solve_file("in2.txt", &mut writer);
        let output_raw = writer.into_inner();
        let output = std::str::from_utf8(&output_raw).unwrap();
        assert!(output.contains("13052"));
        assert!(output.contains("13693"));
    }

    #[test]
    fn day3_compartments() {
        let mut writer = std::io::Cursor::new(vec![]);
        Day3CommonItemInCompartments::solve_file("in3.txt", &mut writer);
        let output_raw = writer.into_inner();
        let output = std::str::from_utf8(&output_raw).unwrap();
        assert!(output.contains("7908"));
    }

    #[test]
    fn day3_groups() {
        let mut writer = std::io::Cursor::new(vec![]);
        Day3CommonItemInGroups::solve_file("in3.txt", &mut writer);
        let output_raw = writer.into_inner();
        let output = std::str::from_utf8(&output_raw).unwrap();
        assert!(output.contains("2838"));
    }

    #[test]
    fn day4_fullyinside() {
        let mut writer = std::io::Cursor::new(vec![]);
        Day4::<OneFullyInsideAnotherSimple>::solve_file("in4.txt", &mut writer);
        let output_raw = writer.into_inner();
        let output = std::str::from_utf8(&output_raw).unwrap();
        assert!(output.contains("450"));
    }

    #[test]
    fn day4_fullyinside_optimized() {
        let mut writer = std::io::Cursor::new(vec![]);
        Day4::<OneFullyInsideAnotherOptimized>::solve_file("in4.txt", &mut writer);
        let output_raw = writer.into_inner();
        let output = std::str::from_utf8(&output_raw).unwrap();
        assert!(output.contains("450"));
    }

    #[test]
    fn day4_overlap() {
        let mut writer = std::io::Cursor::new(vec![]);
        Day4::<Overlap>::solve_file("in4.txt", &mut writer);
        let output_raw = writer.into_inner();
        let output = std::str::from_utf8(&output_raw).unwrap();
        assert!(output.contains("837"));
    }

    #[test]
    fn day5_cratemover9000() {
        let mut writer = std::io::Cursor::new(vec![]);
        Day5::<CrateMover9000>::solve_file("in5.txt", &mut writer);
        let output_raw = writer.into_inner();
        let output = std::str::from_utf8(&output_raw).unwrap();
        assert!(output.contains("RLFNRTNFB"));
    }

    #[test]
    fn day5_cratemover9001() {
        let mut writer = std::io::Cursor::new(vec![]);
        Day5::<CrateMover9001>::solve_file("in5.txt", &mut writer);
        let output_raw = writer.into_inner();
        let output = std::str::from_utf8(&output_raw).unwrap();
        assert!(output.contains("MHQTLJRLB"));
    }

    #[test]
    fn day6_signal() {
        let mut writer = std::io::Cursor::new(vec![]);
        Day6::<4>::solve_file("in6.txt", &mut writer);
        let output_raw = writer.into_inner();
        let output = std::str::from_utf8(&output_raw).unwrap();
        assert!(output.contains("1142"));
    }

    #[test]
    fn day6_message() {
        let mut writer = std::io::Cursor::new(vec![]);
        Day6::<14>::solve_file("in6.txt", &mut writer);
        let output_raw = writer.into_inner();
        let output = std::str::from_utf8(&output_raw).unwrap();
        assert!(output.contains("2803"));
    }

    #[test]
    fn day7_small() {
        let mut writer = std::io::Cursor::new(vec![]);
        Day::<7>::solve_file("in7_small.txt", &mut writer);
        let output_raw = writer.into_inner();
        let output = std::str::from_utf8(&output_raw).unwrap();
        assert!(output.contains("95437"));
        assert!(output.contains("24933642"));
    }

    #[test]
    fn day7() {
        let mut writer = std::io::Cursor::new(vec![]);
        Day::<7>::solve_file("in7.txt", &mut writer);
        let output_raw = writer.into_inner();
        let output = std::str::from_utf8(&output_raw).unwrap();
        assert!(output.contains("1477771"));
        assert!(output.contains("3579501"));
    }

    #[test]
    fn day8_small() {
        let mut writer = std::io::Cursor::new(vec![]);
        Day::<8>::solve_file("in8_small.txt", &mut writer);
        let output_raw = writer.into_inner();
        let output = std::str::from_utf8(&output_raw).unwrap();
        assert!(output.contains("21"));
        assert!(output.contains('8'));
    }

    #[test]
    fn day8() {
        let mut writer = std::io::Cursor::new(vec![]);
        Day::<8>::solve_file("in8.txt", &mut writer);
        let output_raw = writer.into_inner();
        let output = std::str::from_utf8(&output_raw).unwrap();
        assert!(output.contains("1705"));
        assert!(output.contains("371200"));
    }

    #[test]
    fn day9() {
        let mut writer = std::io::Cursor::new(vec![]);
        Day::<9>::solve_file("in9.txt", &mut writer);
        let output_raw = writer.into_inner();
        let output = std::str::from_utf8(&output_raw).unwrap();
        assert!(output.contains("6197"));
        assert!(output.contains("2562"));
    }

    #[test]
    fn day10() {
        let mut writer = std::io::Cursor::new(vec![]);
        Day::<10>::solve_file("in10.txt", &mut writer);
        let output_raw = writer.into_inner();
        let output = std::str::from_utf8(&output_raw).unwrap();
        assert!(output.contains("14220"));
    }
}
