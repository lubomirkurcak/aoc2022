use std::{io::prelude::*, io::BufReader};

use crate::{
    lkc::{
        bijection::Bijection,
        modular::{Modular, ModularArithmetic},
    },
    Problem,
};

#[derive(Debug, Clone)]
struct TangleArray<T> {
    array: Vec<T>,
    bijection: Bijection,
}

impl<T> TangleArray<T> {
    pub fn new(array: Vec<T>) -> Self {
        Self {
            bijection: Bijection::new(array.len()),
            array,
        }
    }

    pub fn get_untangled_element(&self, index: usize) -> &T {
        &self.array[index]
    }

    pub fn get_tangled_element(&self, index: usize) -> &T {
        &self.array[self.bijection.g[index]]
    }
}

impl<T: Copy> TangleArray<T> {
    pub fn collect_tangled(&self) -> Vec<T> {
        let len = self.array.len();
        (0..len).map(|x| *self.get_tangled_element(x)).collect()
    }
}

pub struct Day20<const C: i32, const D: i32>;

impl<const C: i32, const D: i32> Problem for Day20<C, D> {
    fn solve_buffer<T, W>(reader: BufReader<T>, writer: &mut W)
    where
        T: std::io::Read,
        W: std::io::Write,
    {
        let nums = reader
            .lines()
            .map(|x| x.unwrap().parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        let len = nums.len();

        let mut a = TangleArray::new(nums);

        let mut global_offset = 0;

        for _ in 0..C {
            for i in 0..len {
                let mut t = *a.get_untangled_element(i);
                // println!("Picked {} from list!", t);

                let len_minus_1: i32 = (len - 1).try_into().unwrap();
                let decrypt = D % len_minus_1;
                t *= decrypt;

                let accepted_upper_bound = (len_minus_1 + 1) / 2;
                let accepted_lower_bound = -accepted_upper_bound;
                while t < accepted_lower_bound {
                    t += len_minus_1;
                }
                while t > accepted_upper_bound {
                    t -= len_minus_1;
                }

                if t > 0 {
                    for x in 0..t {
                        a.bijection.swap_with_right(i);
                        if a.bijection.f[(x as usize).mod_n(len)] == len - 1 {
                            global_offset.subassign_n(1, len);
                        }
                    }
                } else {
                    for x in 0..-t {
                        a.bijection.swap_with_left(i);
                        if a.bijection.f[(x as usize).mod_n(len)] == 1 {
                            global_offset.addassign_n(1, len);
                        }
                    }
                }
            }
        }

        let mut deciphered = a.collect_tangled();
        deciphered.rotate_right(global_offset);
        let index_of_zero = deciphered.iter().position(|x| *x == 0).unwrap();
        let a = deciphered[(index_of_zero + 1000) % deciphered.len()];
        let b = deciphered[(index_of_zero + 2000) % deciphered.len()];
        let c = deciphered[(index_of_zero + 3000) % deciphered.len()];
        let a = a as i64 * D as i64;
        let b = b as i64 * D as i64;
        let c = c as i64 * D as i64;
        println!("{} + {} + {} = {}", a, b, c, a + b + c);

        writeln!(writer, "{}", a + b + c).unwrap();
    }
}
