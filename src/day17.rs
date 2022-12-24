use crate::lkc::{line::LineV2i32, v2::V2};

pub struct Rock;
impl Rock {
    pub fn width(t: usize) -> i32 {
        match t {
            0 => 4,
            1 => 3,
            2 => 3,
            3 => 1,
            4 => 2,
            _ => panic!(),
        }
    }
    pub fn height(t: usize) -> i32 {
        match t {
            0 => 1,
            1 => 3,
            2 => 3,
            3 => 4,
            4 => 2,
            _ => panic!(),
        }
    }
    pub fn construct(t: usize) -> Vec<LineV2i32> {
        match t {
            0 => vec![LineV2i32::new(V2::new(0, 0), V2::new(3, 0))],
            1 => vec![
                LineV2i32::new(V2::new(0, 1), V2::new(2, 1)),
                LineV2i32::new(V2::new(1, 0), V2::new(1, 2)),
            ],
            2 => vec![
                LineV2i32::new(V2::new(0, 0), V2::new(2, 0)),
                LineV2i32::new(V2::new(2, 0), V2::new(2, 2)),
            ],
            3 => vec![LineV2i32::new(V2::new(0, 0), V2::new(0, 3))],
            4 => vec![
                LineV2i32::new(V2::new(0, 0), V2::new(1, 0)),
                LineV2i32::new(V2::new(0, 1), V2::new(1, 1)),
            ],
            _ => panic!(),
        }
    }
    pub fn left_side() -> Vec<LineV2i32> {
        vec![LineV2i32::new(V2::new(0, 0), V2::new(0, 3))]
    }
    pub fn right_side() -> Vec<LineV2i32> {
        vec![LineV2i32::new(V2::new(6, 0), V2::new(6, 3))]
    }
    fn get_bit_value(x: i32, y: i32) -> u32 {
        assert!(x >= 0);
        assert!(y >= 0);
        assert!(x <= 7);
        assert!(y <= 3);
        assert_ne!(1 << (y * 8 + x), 0);
        1 << (y * 8 + x)
    }
    pub fn construct_blueprint_mask(blueprint: Vec<LineV2i32>) -> u32 {
        let mut bit_rep = 0;
        for line in blueprint {
            for p in line.iter() {
                bit_rep |= Self::get_bit_value(p.x, p.y);
            }
        }
        println!("{:#032b}", bit_rep);

        bit_rep
    }
    pub fn construct_mask(t: usize) -> u32 {
        Self::construct_blueprint_mask(Self::construct(t))
    }
    pub fn left_side_mask() -> u32 {
        Self::construct_blueprint_mask(Self::left_side())
    }
    pub fn right_side_mask() -> u32 {
        Self::construct_blueprint_mask(Self::right_side())
    }
}
