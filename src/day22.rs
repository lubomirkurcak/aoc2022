use std::{
    collections::{HashMap, HashSet},
    io::prelude::*,
    io::BufReader,
};

use crate::{
    lkc::{
        arraynd::Array2d,
        line::{Line, LineV2i32},
        linear_index::LinearIndex,
        modular::{Modi32, ModularAddAssign},
        vector::{V2i32, Vector, V2},
    },
    Problem,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FaceEdge {
    Top,
    Right,
    Bottom,
    Left,
}

impl FaceEdge {
    fn indices(&self) -> (usize, usize) {
        match self {
            FaceEdge::Top => (0, 1),
            FaceEdge::Right => (1, 3),
            FaceEdge::Bottom => (2, 3),
            FaceEdge::Left => (0, 2),
        }
    }

    pub fn iter() -> impl Iterator<Item = Self> {
        [Self::Top, Self::Right, Self::Bottom, Self::Left].into_iter()
    }
}

// NOTE(lubo): Topology of face
//  0------1------> X-axis (RIGHT)
//  |      |
//  |      |
//  2------3
//  |
//  | Y-axis (DOWN)
//  V
type Vert = u8;
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
struct Face([Vert; 4]);
impl Face {
    fn get_edge_ordered(&self, edge: FaceEdge) -> [Vert; 2] {
        let (a, b) = edge.indices();
        [self.0[a], self.0[b]]
    }
    fn get_edge(&self, edge: FaceEdge) -> HashSet<Vert> {
        self.get_edge_ordered(edge).iter().cloned().collect()
    }
    fn is_edge(&self, edge: FaceEdge, vertset: &HashSet<Vert>) -> bool {
        self.get_edge(edge).eq(vertset)
    }
    fn find_edge(&self, vertset: &HashSet<Vert>) -> Option<FaceEdge> {
        FaceEdge::iter().find(|x| self.is_edge(*x, vertset))
    }
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

fn solve_cube_minimap(
    minimap: Array2d<char>,
) -> Vec<(FaceEdge, FaceEdge, i32, LineV2i32, LineV2i32)> {
    let face_ps = minimap.find_all(&'#');
    let minimap_dims: V2i32 = V2::new(minimap.dims).try_into().unwrap();
    let vert_grid_size = minimap_dims + V2::from_xy(1, 1);

    let original_faces = face_ps
        .into_iter()
        .map(|p| {
            let p = p.try_into().unwrap();
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

    let mut faces = original_faces.clone();

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

    let mut edges_to_glue = vec![];
    for face in 0..6 {
        'edge: for edge in FaceEdge::iter() {
            let e = faces[face].get_edge(edge);

            for other in 0..6 {
                if face != other {
                    if let Some(a) = faces[other].find_edge(&e) {
                        // println!("{}'s {:?} shares edge with {}'s {:?}", face, edge, other, a);

                        let rot = (4 + 2 + a as i32 - edge as i32) % 4;
                        // println!("ROTATION IS {} ({} degrees)", rot, rot * 90);

                        let edge_a = original_faces[face].get_edge_ordered(edge);
                        let edge_b = original_faces[other].get_edge_ordered(a);

                        let a0 = vert_grid_size.unindex(edge_a[0] as usize).unwrap();
                        let a1 = vert_grid_size.unindex(edge_a[1] as usize).unwrap();
                        let mut b0 = vert_grid_size.unindex(edge_b[0] as usize).unwrap();
                        let mut b1 = vert_grid_size.unindex(edge_b[1] as usize).unwrap();

                        if a0 == b0 && a1 == b1 {
                        } else {
                            let relabeled_edge_a = faces[face].get_edge_ordered(edge);
                            let mut relabeled_edge_b = faces[other].get_edge_ordered(a);

                            if relabeled_edge_a[0] == relabeled_edge_b[0] {
                                // NOTE(lubo): Already in the right order
                            } else {
                                assert_eq!(relabeled_edge_a[0], relabeled_edge_b[1]);
                                relabeled_edge_b.swap(0, 1);
                                std::mem::swap(&mut b0, &mut b1);
                            }
                            assert_eq!(relabeled_edge_a[0], relabeled_edge_b[0]);

                            edges_to_glue.push((
                                edge,
                                a,
                                rot,
                                Line::new(a0, a1),
                                Line::new(b0, b1),
                            ));
                            println!("[{}, {}]   <-->   [{}, {}]   (Rot {})", a0, a1, b0, b1, rot);
                        }

                        continue 'edge;
                    }
                }
            }
        }
    }

    edges_to_glue
}

pub struct Day22<const B: bool, const C: usize>;

impl<const B: bool, const C: usize> Problem for Day22<B, C> {
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
        let mut map = Array2d {
            data: map.clone().collect::<Vec<_>>().concat().chars().collect(),
            dims: [map_width, map.clone().count()],
            dim_strides: [1, map_width],
        };

        let mut minimap = Array2d::with_dimensions(map.width() / C, map.height() / C, '.');
        for y in 0..minimap.height() {
            for x in 0..minimap.width() {
                let p = V2::from_xy(x as i32, y as i32);
                let q = p * C.try_into().unwrap();
                let v = *map.get(q).unwrap();
                if v != ' ' {
                    minimap.set(p, '#');
                }
            }
        }
        println!("{}", minimap);
        let mut teleport_stripes = vec![];
        let edges_to_glue = solve_cube_minimap(minimap);
        let mut map = map.padded(1, ' ');
        for (index, (side1, side2, rot, a, b)) in edges_to_glue.into_iter().enumerate() {
            let mut a = a.scale(C.try_into().unwrap()).offset(Vector::all(1));
            let mut b = b.scale(C.try_into().unwrap()).offset(Vector::all(1));

            if side1 == FaceEdge::Left {
                a = a.offset(V2::from_xy(-1, 0));
            }
            if side1 == FaceEdge::Top {
                a = a.offset(V2::from_xy(0, -1));
            }
            if side2 == FaceEdge::Left {
                b = b.offset(V2::from_xy(-1, 0));
            }
            if side2 == FaceEdge::Top {
                b = b.offset(V2::from_xy(0, -1));
            }

            assert!(a.start.x() <= a.end.x() && a.start.y() <= a.end.y());

            if a.start.x() == a.end.x() {}

            if b.start.x() <= b.end.x() && b.start.y() <= b.end.y() {
                // ok
            } else {
                println!("B is inverted :(");
                b = b.offset(b.delta().elementwise_unary(|x| x.signum()));
            }

            map.draw_line::<false>(a, 'O');
            map.draw_line::<false>(b, 'O');
            println!("{}", map);

            teleport_stripes.push((rot, a, b));
        }
        println!("{}", map);

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
                    let mut probe: V2i32 = pos;
                    let mut probe_rot = rotation;
                    'steps: for _step in 0..w {
                        'this_step: loop {
                            probe.addassign_n(rotation_to_direction(probe_rot), map_dims);
                            match map.get(probe).unwrap() {
                                '.' => {
                                    pos = probe;
                                    rotation = probe_rot;
                                    draw_map
                                        .set(pos, ['>', 'v', '<', '^'][rotation.get() as usize]);
                                    break 'this_step;
                                }
                                'O' => {
                                    if B {
                                        for (rot, from, to) in teleport_stripes.iter() {
                                            if let Some(from_index) =
                                                from.iter::<false>().position(|x| x == probe)
                                            {
                                                probe = to.iter::<false>().nth(from_index).unwrap();
                                                probe_rot += Modi32::new(*rot);
                                                probe.addassign_n(
                                                    rotation_to_direction(probe_rot),
                                                    map_dims,
                                                );

                                                match map.get(probe).unwrap() {
                                                    '.' => (),
                                                    '#' => break 'steps,
                                                    _ => panic!(),
                                                }

                                                pos = probe;
                                                rotation = probe_rot;
                                                draw_map.set(
                                                    pos,
                                                    ['>', 'v', '<', '^'][rotation.get() as usize],
                                                );
                                                break 'this_step;
                                            }
                                        }
                                        panic!("Unhandled teleport tile!");
                                    }
                                }
                                '#' => break 'steps,
                                ' ' => (),
                                _ => (),
                            }
                        }
                    }
                }
            }
            // println!("{}", draw_map);
            println!("Pos {} Rot {}", pos, rotation);
        }

        println!("{}", draw_map);

        println!("Pos {} Rot {}", pos, rotation);
        let row = pos.y();
        let col = pos.x();
        let rot = match rotation.get() {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            _ => panic!(),
        };
        let result = 1000 * row + 4 * col + rot;
        println!("Row {} Col {} Rot {} Final {}", row, col, rot, result);

        writeln!(writer, "{}", result).unwrap();
    }
}
