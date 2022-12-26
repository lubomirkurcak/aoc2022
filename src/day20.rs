use std::{io::prelude::*, io::BufReader};

use crate::{lkc::modular::ModularArithmetic, Day, Problem};

impl Problem for Day<20> {
    fn solve_buffer<T, W>(reader: BufReader<T>, writer: &mut W)
    where
        T: std::io::Read,
        W: std::io::Write,
    {
        let mut lines = reader
            .lines()
            .map(|x| x.unwrap().parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        let mut lines = vec![2, 1, -3, 3, 2, 0, 4];

        let len = lines.len();
        let indices = &mut (0..lines.len()).collect::<Vec<_>>()[..];
        let coindices = &mut (0..lines.len()).collect::<Vec<_>>()[..];

        println!("{:?}", lines);
        println!("{:?}", indices);

        for original_index in 0..len {
            let mut index = indices[original_index];

            assert_eq!(coindices[index], original_index);

            let a = *lines.get(index).unwrap();

            println!("Picked {} from list.", a);

            if a > 0 {
                let mut last = index;
                for i in 0..a {
                    let b: usize = i.try_into().unwrap();
                    let next = index.add_n(1 + b, len);
                    indices[index].addassign_n(1, len);
                    indices[next].subassign_n(1, len);
                    coindices[index].addassign_n(1, len);
                    coindices[next].subassign_n(1, len);
                    lines.swap(last, next);

                    last = next;

                    for o in 0..len {
                        assert_eq!(coindices[indices[o]], o);
                        assert_eq!(indices[coindices[o]], o);
                    }
                }
            }

            println!("{:?}", lines);
            println!("{:?}", indices);
        }

        writeln!(writer, "{}", 0).unwrap();
    }
}
