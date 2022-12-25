use std::{cmp::max, collections::HashSet, io::prelude::*, io::BufReader};

use crate::{
    lkc::{aabb::Aabb, geometric_traits::CoverObject, vector::V3},
    Day, Problem,
};

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
enum Normals {
    X,
    Y,
    Z,
}

// impl Problem for Day<1801> {
//     fn solve_buffer<T, W>(reader: BufReader<T>, writer: &mut W)
//     where
//         T: std::io::Read,
//         W: std::io::Write,
//     {
//         let mut result = 0;
//
//         let mut open_sides = 0;
//         let mut sides = HashSet::new();
//
//         for line in reader.lines().map(|x| x.unwrap()) {
//             let a = line.split(',').collect::<Vec<_>>();
//             let x: i32 = a[0].parse().unwrap();
//             let y: i32 = a[1].parse().unwrap();
//             let z: i32 = a[2].parse().unwrap();
//
//             let mut g = |x| {
//                 if sides.contains(&x) {
//                     open_sides -= 1;
//                 } else {
//                     open_sides += 1;
//                     sides.insert(x);
//                 }
//             };
//
//             g((x, y, z, Normals::X));
//             g((x, y, z, Normals::Y));
//             g((x, y, z, Normals::Z));
//             g((x + 1, y, z, Normals::X));
//             g((x, y + 1, z, Normals::Y));
//             g((x, y, z + 1, Normals::Z));
//
//             result = max(result, line.len());
//         }
//
//         println!("{}", open_sides);
//
//         writeln!(writer, "{}", result).unwrap();
//     }
// }

impl Problem for Day<1801> {
    fn solve_buffer<T, W>(reader: BufReader<T>, writer: &mut W)
    where
        T: std::io::Read,
        W: std::io::Write,
    {
        let mut result = 0;

        let mut open_sides = 0;
        let mut sides = HashSet::new();

        let mut side_x = HashSet::new();
        let mut side_y = HashSet::new();
        let mut side_z = HashSet::new();

        let mut aabb: Option<Aabb<3, _>> = None;

        for line in reader.lines().map(|x| x.unwrap()) {
            let a = line.split(',').collect::<Vec<_>>();
            let x: i32 = a[0].parse().unwrap();
            let y: i32 = a[1].parse().unwrap();
            let z: i32 = a[2].parse().unwrap();

            let v = V3::from_xyz(x, y, z);

            match aabb.as_mut() {
                Some(aabb) => aabb.cover(&v),
                None => aabb = Some(Aabb::new(v, v)),
            }

            if !side_x.contains(&(y, z)) {
                side_x.insert((y, z));
            }
            if !side_y.contains(&(x, z)) {
                side_y.insert((x, z));
            }
            if !side_z.contains(&(x, y)) {
                side_z.insert((x, y));
            }

            let mut g = |x| {
                if sides.contains(&x) {
                    open_sides -= 1;
                } else {
                    open_sides += 1;
                    sides.insert(x);
                }
            };

            g((x, y, z, Normals::X));
            g((x, y, z, Normals::Y));
            g((x, y, z, Normals::Z));
            g((x + 1, y, z, Normals::X));
            g((x, y + 1, z, Normals::Y));
            g((x, y, z + 1, Normals::Z));

            result = max(result, line.len());
        }

        println!("Boundary area {}", open_sides);
        println!(
            "Shadow boundary area {}",
            2 * (side_x.len() + side_y.len() + side_z.len())
        );
        println!("Aabb {:?}", aabb);

        writeln!(writer, "{}", result).unwrap();
    }
}
