use std::{cmp::Ordering, fmt::Display, hash::Hash, io::BufReader, vec};

use crate::{
    day16::{PointTrait, RoomId, Rooms},
    lkc::{
        explore::{Exploration, ExploreSignals, PointKeyValue},
        geometric_traits::IterateNeighbours,
    },
    Day, Problem,
};

// type RoomId = i32;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Point {
    room_id: RoomId,
    time: i32,
    open_valves: u64,
    pressure_released: u64,
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Open valves {:#064b}, Room {}, Time {}, Pressure released: {}",
            self.open_valves, self.room_id, self.time, self.pressure_released,
        )
    }
}

impl PointKeyValue for Point {
    type K = (u64, i32);
    type V = (i32, u64);
    fn get_key(&self) -> Self::K {
        (self.open_valves, self.room_id)
    }

    fn get_value(&self) -> Self::V {
        (self.time, self.pressure_released)
    }

    fn compare_values(a: &Self::V, b: &Self::V) -> Option<Ordering> {
        let (a_time, a_pressure) = a;
        let (b_time, b_pressure) = b;
        if a_time >= b_time && a_pressure <= b_pressure {
            return Some(Ordering::Less);
        }

        if a_time <= b_time && a_pressure >= b_pressure {
            return Some(Ordering::Greater);
        }

        None
    }
}

impl PointTrait for Point {
    fn time_left(&self) -> i32 {
        30 - self.time
    }

    fn get_open_valves(&self) -> u64 {
        self.open_valves
    }
}

impl Point {
    pub fn initial(room_id: RoomId, rooms: &Rooms) -> Self {
        Self {
            room_id,
            time: 0,
            open_valves: Self::initial_valve_state(rooms),
            pressure_released: 0,
        }
    }
}

impl IterateNeighbours for Point {
    type Context = Exploration<Self, Rooms>;

    fn neighbours(&self, context: &Self::Context) -> Vec<Self> {
        let mut options = vec![];
        let rooms = &context.structure;

        if self.time_left() > 0 {
            rooms
                .collection
                .get(&self.room_id)
                .unwrap()
                .floyd_warshall_connections
                .iter()
                .for_each(|(p, distance)| {
                    let valve_open_time = self.time + distance + 1;
                    let valve_open_time_left = 30 - valve_open_time;
                    if valve_open_time_left > 0 {
                        let valve_open_value =
                            Self::open_valve_value(*p, valve_open_time_left, rooms);
                        let new_valve_state = self.open_valves | Self::get_valve_mask(*p);
                        if new_valve_state != self.open_valves && valve_open_time <= 30 {
                            options.push(Self {
                                room_id: *p,
                                time: valve_open_time,
                                open_valves: new_valve_state,
                                pressure_released: self.pressure_released + valve_open_value,
                            })
                        }
                    }
                });
        }

        options
    }
}

impl Problem for Day<1601> {
    fn solve_buffer<T, W>(reader: BufReader<T>, writer: &mut W)
    where
        T: std::io::Read,
        W: std::io::Write,
    {
        let rooms = Rooms::from_buffer(reader);
        let exp = Exploration::new(rooms);
        let mut max_pressure_released = 0;

        exp.explore_avoid_worse(
            Point::initial(0, &exp.structure),
            |p| {
                let state_potential = p.state_potential_overestimate(&exp.structure);

                if state_potential == 0 {
                    return ExploreSignals::Skip;
                }
                if p.pressure_released + state_potential <= max_pressure_released {
                    return ExploreSignals::Skip;
                }

                if p.pressure_released > max_pressure_released {
                    max_pressure_released = p.pressure_released;
                }

                ExploreSignals::Explore
            },
            |_p| true,
        );

        println!("MAX PRESSURE {}", max_pressure_released);

        writeln!(writer, "Result: {}", max_pressure_released).unwrap();
    }
}
