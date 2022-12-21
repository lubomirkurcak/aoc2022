use std::{hash::Hash, io::BufReader, vec};

use crate::{
    day16::{PointTrait, Rooms},
    lkc::{
        explore::{Exploration, ExploreSignals},
        geometric_traits::IterateNeighbours,
    },
    Day, Problem,
};

use crate::day16::RoomId;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Point2 {
    me_p: RoomId,
    elephant_p: RoomId,
    me_finish_task_time: i32,
    elephant_finish_task_time: i32,
    open_valves: u64,
    pressure_released: u64,
}

impl PointTrait for Point2 {
    fn time_left(&self) -> i32 {
        26 - std::cmp::min(self.elephant_finish_task_time, self.me_finish_task_time)
    }

    fn get_open_valves(&self) -> u64 {
        self.open_valves
    }
}

impl Point2 {
    pub fn initial(room_id: RoomId, rooms: &Rooms) -> Self {
        Self {
            open_valves: Self::initial_valve_state(rooms),
            pressure_released: 0,
            me_p: room_id,
            elephant_p: room_id,
            me_finish_task_time: 0,
            elephant_finish_task_time: 0,
        }
    }
}

impl IterateNeighbours for Point2 {
    type Context = Exploration<Self, Rooms>;

    fn neighbours(&self, context: &Self::Context) -> Vec<Self> {
        let mut options = vec![];
        let rooms = &context.structure;

        let (time, my_action_next, room_id) =
            if self.me_finish_task_time < self.elephant_finish_task_time {
                (self.me_finish_task_time, true, self.me_p)
            } else {
                (self.elephant_finish_task_time, false, self.elephant_p)
            };

        if self.time_left() > 0 {
            rooms
                .collection
                .get(&room_id)
                .unwrap()
                .floyd_warshall_connections
                .iter()
                .for_each(|(p, distance)| {
                    let valve_open_time = time + distance + 1;
                    let valve_open_time_left = 26 - valve_open_time;
                    if valve_open_time_left > 0 {
                        let new_valve_state = self.open_valves | Self::get_valve_mask(*p);
                        if new_valve_state != self.open_valves {
                            let mut child = *self;

                            child.open_valves = new_valve_state;
                            child.pressure_released +=
                                Self::open_valve_value(*p, valve_open_time_left, rooms);

                            if my_action_next {
                                child.me_p = *p;
                                child.me_finish_task_time = valve_open_time;
                            } else {
                                child.elephant_p = *p;
                                child.elephant_finish_task_time = valve_open_time;
                            }

                            options.push(child);
                        }
                    }
                });
        }

        options
    }
}

impl Problem for Day<1602> {
    fn solve_buffer<T, W>(reader: BufReader<T>, writer: &mut W)
    where
        T: std::io::Read,
        W: std::io::Write,
    {
        let rooms = Rooms::from_buffer(reader);
        let exp = Exploration::new(rooms);
        let mut max_pressure_released = 0;

        exp.explore(
            Point2::initial(0, &exp.structure),
            |p| {
                let state_potential = p.state_potential_overestimate(&exp.structure);

                if state_potential == 0 {
                    return ExploreSignals::Skip;
                }
                if p.pressure_released + state_potential <= max_pressure_released {
                    return ExploreSignals::Skip;
                }

                if p.pressure_released > max_pressure_released {
                    println!("New best PRESSURE {} @ {:?}", max_pressure_released, p);
                    max_pressure_released = p.pressure_released;
                }

                ExploreSignals::Explore
            },
            |_p| true,
        );

        println!("MAX ELEPHANT PRESSURE {}", max_pressure_released);

        writeln!(writer, "Result: {}", max_pressure_released).unwrap();
    }
}