use std::{collections::HashSet, io::prelude::*, io::BufReader, str::FromStr};

use crate::{
    lkc::{
        aabb::Aabb,
        arraynd::Array3d,
        explore::{Exploration, ExploreSignals},
        geometric_traits::CoverObject,
        vector::{Scalar, V3},
    },
    Day, Problem,
};

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
enum Normals {
    X,
    Y,
    Z,
}

impl Problem for Day<1801> {
    fn solve_buffer<T, W>(reader: BufReader<T>, writer: &mut W)
    where
        T: std::io::Read,
        W: std::io::Write,
    {
        let mut open_sides = 0;
        let mut sides = HashSet::new();

        for line in reader.lines().map(|x| x.unwrap()) {
            let v = V3::<i32>::from_str(&line).unwrap();
            let x = v.x();
            let y = v.y();
            let z = v.z();

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
        }

        write!(writer, "{}", open_sides).unwrap();
    }
}

impl Problem for Day<1802> {
    fn solve_buffer<T, W>(reader: BufReader<T>, writer: &mut W)
    where
        T: std::io::Read,
        W: std::io::Write,
    {
        let mut occ = HashSet::new();
        let mut aabb: Option<Aabb<3, _>> = None;

        for line in reader.lines().map(|x| x.unwrap()) {
            let v = V3::<i32>::from_str(&line).unwrap();

            occ.insert(v);

            match aabb.as_mut() {
                Some(aabb) => aabb.cover(&v),
                None => aabb = Some(Aabb::new(v, v)),
            }
        }

        let min = aabb.unwrap().min;
        let dim =
            aabb.unwrap().dim() + V3::from_xyz(1, 1, 1) + Scalar::new(2) * V3::from_xyz(1, 1, 1);

        let unknown = '.';
        let lava = '#';

        let mut map = Array3d::new(dim.values, unknown);
        for x in occ.iter() {
            map.set(*x - min + V3::from_xyz(1, 1, 1), lava);
        }

        // NOTE(lubo): We know this point is outside. (It lies on the 1 block boundary we created)
        let start = V3::from_xyz(0, 0, 0);

        let mut result = 0;
        let mut exp = Exploration::new(map);
        exp.explore_avoid_identical(
            start,
            |a, map| {
                debug_assert_ne!(map.get(*a).unwrap(), &lava);
                ExploreSignals::Explore
            },
            |_p, a, map| {
                let v = map.get(*a).unwrap();
                if v == &lava {
                    result += 1;
                }
                v == &unknown
            },
        );

        writeln!(writer, "{}", result).unwrap();
    }
}
