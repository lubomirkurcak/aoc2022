use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt::Display,
    io::prelude::*,
    io::BufReader,
};

use crate::{
    lkc::{
        arraynd::Array2d,
        explore::{Exploration, ExploreSignals},
        geometric_algebra::Multivector3,
        linear_index::LinearIndex,
        modular::{Modi32, ModularAdd, ModularAddAssign},
        sketch::StackBag,
        vector::{V2i32, V2usize, Vector, V2, V3},
    },
    Day, Problem,
};

#[derive(Debug)]
enum Instruction {
    TurnLeft,
    TurnRight,
    Walk(i32),
}

fn rotation_to_direction(rotation: Modi32<4>) -> V2<i32> {
    match rotation.get() % 4 {
        0 => V2::from_xy(1, 0),
        1 => V2::from_xy(0, 1),
        2 => V2::from_xy(-1, 0),
        3 => V2::from_xy(0, -1),
        _ => panic!(),
    }
}

fn parse_instructions(s: &str) -> Vec<Instruction> {
    use Instruction::*;
    let mut results = vec![];
    let mut acc = vec![];
    for c in s.chars() {
        if ('0'..='9').contains(&c) {
            acc.push(c);
        } else {
            if !acc.is_empty() {
                results.push(Walk(acc.iter().collect::<String>().parse().unwrap()));
                acc.clear();
            }

            results.push(match c {
                'L' => TurnLeft,
                'R' => TurnRight,
                _ => panic!(),
            });
        }
    }
    if !acc.is_empty() {
        results.push(Walk(acc.iter().collect::<String>().parse().unwrap()));
        acc.clear();
    }
    results
}

type Vert = u8;
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Face([Vert; 4]);
impl Face {
    fn has_vert(&self, vert: &Vert) -> bool {
        self.0.iter().any(|x| x == vert)
    }
    fn get_opposite_vert_mut(&mut self, vert: &Vert) -> Option<&mut Vert> {
        let a = self.iter().position(|x| x == vert);
        match a {
            Some(a) => Some(
                &mut self.0[match a {
                    0 => 3,
                    1 => 2,
                    2 => 1,
                    3 => 0,
                    _ => panic!(),
                }],
            ),
            None => None,
        }
    }
    fn get_opposite_vert(&self, vert: &Vert) -> Option<&Vert> {
        let a = self.iter().position(|x| x == vert);
        match a {
            Some(a) => Some(
                &self.0[match a {
                    0 => 3,
                    1 => 2,
                    2 => 1,
                    3 => 0,
                    _ => panic!(),
                }],
            ),
            None => None,
        }
    }
    fn iter(&self) -> impl Iterator<Item = &Vert> + '_ {
        self.0.iter()
    }
    fn to_set(&self) -> HashSet<Vert> {
        self.iter().cloned().collect()
    }
    fn intersection(&self, other: &Self) -> HashSet<Vert> {
        self.to_set()
            .intersection(&other.to_set())
            .cloned()
            .collect()
    }
    fn shares_edge(&self, other: &Self) -> bool {
        self.intersection(&other).len() == 2
    }
}

fn cube_wrap_rule_1(faces: &[Face]) -> HashMap<Vert, Vert> {
    let mut relabels = HashMap::new();

    let len = faces.len();
    for i in 0..len {
        for j in 0..len {
            if i == j {
                continue;
            }
            for k in 0..j {
                if faces[i].shares_edge(&faces[j]) && faces[i].shares_edge(&faces[k]) {
                    let shared = faces[j].intersection(&faces[k]);
                    // Rule 1
                    if shared.len() == 1 {
                        // let shared = shared.drain().next().unwrap();

                        let ij = faces[i].intersection(&faces[j]);
                        let ik = faces[i].intersection(&faces[k]);

                        let ij_not_k = ij.difference(&shared).next().unwrap();
                        let ik_not_j = ik.difference(&shared).next().unwrap();

                        let merge1 = *faces[j].get_opposite_vert(ij_not_k).unwrap();
                        let merge2 = *faces[k].get_opposite_vert(ik_not_j).unwrap();

                        relabels.insert(merge1.max(merge2), merge1.min(merge2));

                        println!(
                            "Rule one for faces {}, {}, {}. Joining {}-{} and {}-{}",
                            i,
                            j,
                            k,
                            shared.iter().next().unwrap(),
                            merge1,
                            shared.iter().next().unwrap(),
                            merge2,
                        );
                    }
                }
            }
        }
    }

    relabels
}

fn cube_wrap(mut minimap: Array2d<char>) {
    let face_ps = minimap.find_all(&'#');

    let mut faces = face_ps
        .into_iter()
        .map(|p| {
            let p = p.try_into().unwrap();
            let vert_grid_size = V2::new(minimap.dims) + V2::from_xy(1, 1);
            let a = vert_grid_size.index_unchecked(p).unwrap();
            let b = vert_grid_size
                .index_unchecked(p + V2::from_xy(1, 0))
                .unwrap();
            let c = vert_grid_size
                .index_unchecked(p + V2::from_xy(0, 1))
                .unwrap();
            let d = vert_grid_size
                .index_unchecked(p + V2::from_xy(1, 1))
                .unwrap();

            let a: Vert = a.try_into().unwrap();
            let b: Vert = b.try_into().unwrap();
            let c: Vert = c.try_into().unwrap();
            let d: Vert = d.try_into().unwrap();

            Face([a, b, c, d])
        })
        .collect::<Vec<_>>();

    loop {
        let relabels = cube_wrap_rule_1(&faces[..]);
        if relabels.is_empty() {
            break;
        }

        faces.iter_mut().for_each(|face| {
            face.0.iter_mut().for_each(|vert| {
                if relabels.contains_key(vert) {
                    *vert = *relabels.get(vert).unwrap();
                }
            })
        });
    }
}

fn solve_cube_minimap(mut minimap: Array2d<char>) {
    println!("                 ");
    println!("      G---------H");
    println!("     /|        /|");
    println!("    / |       / |");
    println!("   C---------D  |");
    println!("   |  E------|--F");
    println!("   | /       | / ");
    println!("   |/        |/  ");
    println!("   A---------B   ");
    println!("                 ");
    println!(" Possible Faces: ");
    println!(" ABCD            ");
    println!(" ABEF            ");
    println!(" ACEG            ");
    println!(" BDFH            ");
    println!(" CDGH            ");
    println!(" EFGH            ");
    println!("                 ");
    cube_wrap(minimap);
}

impl Problem for Day<22> {
    fn solve_buffer<T, W>(reader: BufReader<T>, writer: &mut W)
    where
        T: std::io::Read,
        W: std::io::Write,
    {
        let lines = reader.lines().map(|x| x.unwrap()).collect::<Vec<_>>();
        let split = lines.iter().position(|x| x.is_empty()).unwrap();
        let map = &lines[0..split];

        let map_width = map.iter().map(|x| x.len()).max().unwrap();
        let map = map.iter().map(|x| {
            let a = format!("{:1$}", x, map_width);
            a
        });
        assert!(map.clone().all(|x| x.len() == map_width));
        let map = Array2d {
            data: map.clone().collect::<Vec<_>>().concat().chars().collect(),
            dims: [map_width, map.clone().count()],
            dim_strides: [1, map_width],
        };

        let mut minimap = Array2d::with_dimensions(map.width() / 50, map.height() / 50, '.');
        for y in 0..minimap.height() {
            for x in 0..minimap.width() {
                let p = V2::from_xy(x as i32, y as i32);
                let q = p * 50;
                let v = *map.get(q).unwrap();
                if v != ' ' {
                    minimap.set(p, '#');
                }
            }
        }
        println!("{}", minimap);

        solve_cube_minimap(minimap);

        let instructions = parse_instructions(&lines[split..].to_vec().concat());

        let map_dims = V2::from_xy(
            map.dims[0].try_into().unwrap(),
            map.dims[1].try_into().unwrap(),
        );
        let mut draw_map = map.clone();
        let mut pos = map.find(&'.').unwrap();
        let mut rotation = Modi32::new(0);
        for instruction in instructions {
            println!("{:?}", instruction);
            match instruction {
                Instruction::TurnLeft => rotation += Modi32::new(-1),
                Instruction::TurnRight => rotation += Modi32::new(1),
                Instruction::Walk(w) => {
                    let delta = rotation_to_direction(rotation);
                    let mut probe: V2i32 = pos;
                    'outer: for _step in 0..w {
                        loop {
                            probe.addassign_n(delta, map_dims);
                            match map.get(probe).unwrap() {
                                '.' => {
                                    pos = probe;
                                    draw_map
                                        .set(pos, ['>', 'v', '<', '^'][rotation.get() as usize]);
                                    break;
                                }
                                '#' => break 'outer,
                                ' ' => (),
                                _ => panic!(),
                            }
                        }
                    }
                }
            }
            // println!("{}", draw_map);
            // println!("Pos {} Rot {}", pos, rotation);
        }

        println!("{}", draw_map);

        println!("Pos {} Rot {}", pos, rotation);
        let row = pos.y() + 1;
        let col = pos.x() + 1;
        let rot = match rotation.get() {
            0 => 0,
            1 => 3,
            2 => 2,
            3 => 1,
            _ => panic!(),
        };
        println!(
            "Row {} Col {} Rot {} Final {}",
            row,
            col,
            rot,
            1000 * row + 4 * col + rot
        );

        let result = 0;
        writeln!(writer, "{}", result).unwrap();
    }
}
