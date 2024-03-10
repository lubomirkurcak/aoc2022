use std::{collections::HashSet, io::prelude::*, io::BufReader};

use crate::Problem;
use lk_math::{interval::InclusiveMin, interval_set::IntervalSet, prelude::*};

fn parse_sensors_data<T>(reader: BufReader<T>) -> Vec<(V2<i32>, V2<i32>)>
where
    T: std::io::Read,
{
    reader
        .lines()
        .map(|x| x.unwrap())
        .map(|line| {
            let a = line.split("Sensor at x=").collect::<Vec<_>>()[1];
            let a = a.splitn(2, ", y=").collect::<Vec<_>>();
            let sx: i32 = a[0].trim().parse().unwrap();
            let a = a[1].split(": closest beacon is at x=").collect::<Vec<_>>();
            let sy: i32 = a[0].trim().parse().unwrap();
            let s = V2::from_xy(sx, sy);
            let a = a[1].split(", y=").collect::<Vec<_>>();
            let bx: i32 = a[0].trim().parse().unwrap();
            let by: i32 = a[1].trim().parse().unwrap();
            let b = V2::from_xy(bx, by);
            (s, b)
        })
        .collect::<Vec<_>>()
}

fn unzip_vec_of_2tuple<T: Clone>(v: &[(T, T)]) -> (Vec<T>, Vec<T>) {
    // v.iter().cloned().unzip::<_, _, Vec<_>, Vec<_>>()
    v.iter().cloned().unzip()
}

pub struct DefinitelyNoBeaconsAtLine<const Y: i32>;

pub struct FindTheLoneOutOfRangeTile<const C: i32>;

pub struct Day15<T> {
    phantom: std::marker::PhantomData<T>,
}

impl<const Y: i32> Problem for Day15<DefinitelyNoBeaconsAtLine<Y>> {
    fn solve_buffer<T, W>(reader: BufReader<T>, writer: &mut W)
    where
        T: std::io::Read,
        W: std::io::Write,
    {
        let a = parse_sensors_data(reader);
        let (sensors, beacons) = unzip_vec_of_2tuple(&a);
        let mut objects = sensors;
        objects.extend(beacons.iter());

        let objects_on_line = objects
            .iter()
            .filter(|p| p.y() == Y)
            .map(|p| p.x())
            .collect::<HashSet<_>>();

        let mut interval_set = IntervalSet::new();

        for (s, b) in a.iter() {
            let range = s.manhattan_distance(b);
            let distance_to_line = (s.y() - Y).abs();
            let range_remaining = range - distance_to_line;
            if range_remaining > 0 {
                let range_x_min = s.x() - range_remaining;
                let range_x_max = s.x() + range_remaining;
                let covering_line_x = range_x_min..range_x_max + 1;
                interval_set.union(covering_line_x);
            }
        }

        let cant_be = interval_set.measure() as usize - objects_on_line.len();

        writeln!(writer, "{}", cant_be).unwrap();
    }
}

impl<const C: i32> Problem for Day15<FindTheLoneOutOfRangeTile<C>> {
    fn solve_buffer<T, W>(reader: BufReader<T>, writer: &mut W)
    where
        T: std::io::Read,
        W: std::io::Write,
    {
        let a = parse_sensors_data(reader);
        let (sensors, beacons) = unzip_vec_of_2tuple(&a);
        let mut objects = sensors;
        objects.extend(beacons.iter());

        /*
        NOTE(lubo): This approach works, but is not good.
        We should do simulated annealing/hill climbing instead (particles
        pushed away from sensors if they are within range until they find
        a spot where they remain in rest)
        */
        for line_y in 0..C {
            let mut interval_set = IntervalSet::new();

            for (s, b) in a.iter() {
                let range = s.manhattan_distance(b);
                let distance_to_line = (s.y() - line_y).abs();
                let range_remaining = range - distance_to_line;
                if range_remaining > 0 {
                    let range_x_min = s.x() - range_remaining;
                    let range_x_max = s.x() + range_remaining;
                    let covering_line_x = range_x_min..range_x_max + 1;
                    interval_set.union(covering_line_x);
                }
            }

            interval_set.intersect(0..C);

            if line_y % 100000 == 0 {
                println!("Line Y = {}", line_y);
            }

            let measure = interval_set.measure();
            if measure != C {
                println!(
                    "Line Y {} Measure {} Interval Set {:?} Negated in bounds {:?}",
                    line_y,
                    measure,
                    interval_set,
                    interval_set.negation_within_bounds()
                );

                let negated = interval_set.negation_within_bounds();
                assert_eq!(negated.measure(), 1);
                let x = *negated.intervals[0].inclusive_min();
                let tuning_frequency = 4000000i64 * x as i64 + line_y as i64;

                println!(
                    "x: {} y: {} tuning_frequency: {}",
                    x, line_y, tuning_frequency
                );

                write!(writer, "{}", tuning_frequency).unwrap();
            }
        }
    }
}
