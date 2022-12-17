use std::{io::prelude::*, io::BufReader, str::FromStr};

use crate::lkc::transformations::{Transform, Translation};

use crate::lkc::geometric_traits::CoverObject;
use crate::{
    lkc::{aabb::AABB2, v2::Scalar, v2::V2},
    Day, Problem,
};

impl Problem for Day<14> {
    fn solve_buffer<T, W>(reader: BufReader<T>, writer: &mut W)
    where
        T: std::io::Read,
        W: std::io::Write,
    {
        let mut result = 0;

        let sand_source = V2::new(500, 0);
        let mut aabb = AABB2::new(sand_source, sand_source);

        let lines = reader.lines().map(|x| x.unwrap()).collect::<Vec<_>>();
        for line in lines {
            for coords in line.split("->") {
                let coords = V2::<i32>::from_str(coords).unwrap();
                aabb.cover(&coords);
                println!("{:?}", coords);
            }
        }
        println!("{:?}", aabb);
        println!("{:?}", aabb.dim());
        //let map_dim = aabb.dim() + V2::new(1, 1) + V2::new(1, 1) * 2;
        let map_dim = aabb.dim() + V2::new(1, 1) + Scalar::new(2) * V2::new(1, 1);
        println!("{:?}", map_dim);
        let t = Translation::new(aabb.min - V2::new(1, 1));
        println!("{:?}", t);

        println!("{:?}", t.inverse_transform(aabb.min));
        println!("{:?}", t.inverse_transform(aabb.max));

        writeln!(writer, "Result: {}", result).unwrap();
    }
}
