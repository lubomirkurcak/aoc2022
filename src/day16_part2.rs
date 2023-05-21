use std::{cmp::Ordering, hash::Hash, io::BufReader, vec};

use crate::{
    day16::{PointTrait, Rooms},
    Day, Problem,
};
use lk_math::{
    explore::{Exploration, ExploreSignals, PointKeyValue},
    geometric_traits::IterateNeighbours,
    sketch::StackBag,
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
        Self::get_total_time()
            - std::cmp::min(self.elephant_finish_task_time, self.me_finish_task_time)
    }

    fn get_open_valves(&self) -> u64 {
        self.open_valves
    }

    fn get_total_time() -> i32 {
        26
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

    fn state_potential_overestimate(&self, rooms: &Rooms) -> u64 {
        let mut res = 0;
        // NOTE(lubo): Not TOTALLY sure about these -1's here. They could underestimate in some cases. It worked for our input though.
        let mut tl0 = Self::get_total_time() - self.me_finish_task_time - 1;
        let mut tl1 = Self::get_total_time() - self.elephant_finish_task_time - 1;
        let mut ps = self.get_releasable_pressures(rooms);
        ps.sort();
        ps.reverse();

        for p in ps {
            if tl0 > tl1 {
                res += p * tl0;
                tl0 -= 2;
            } else {
                res += p * tl1;
                tl1 -= 2;
            }
        }

        if res > 0 {
            res as u64
        } else {
            0
        }
    }
}

impl PointKeyValue for Point2 {
    type K = (u64, i32, i32);
    type V = (u64, i32, i32);

    fn get_key(&self) -> Self::K {
        (self.open_valves, self.me_p, self.elephant_p)
    }

    fn get_value(&self) -> Self::V {
        (
            self.pressure_released,
            self.me_finish_task_time,
            self.elephant_finish_task_time,
        )
    }

    fn compare_values(_k: &Self::K, a: &Self::V, b: &Self::V) -> Option<Ordering> {
        if a.0 <= b.0 && a.1 >= b.1 && a.2 >= b.2 {
            return Some(Ordering::Less);
        }
        if a.0 >= b.0 && a.1 <= b.1 && a.2 <= b.2 {
            return Some(Ordering::Greater);
        }

        None
    }
}

impl IterateNeighbours<Rooms> for Point2 {
    fn neighbours(&self, context: &Rooms) -> Vec<Self> {
        let mut options = vec![];

        let (time, my_action_next, room_id) =
            if self.me_finish_task_time < self.elephant_finish_task_time {
                (self.me_finish_task_time, true, self.me_p)
            } else {
                (self.elephant_finish_task_time, false, self.elephant_p)
            };

        if self.time_left() > 0 {
            context
                .collection
                .get(&room_id)
                .unwrap()
                .floyd_warshall_connections
                .iter()
                .for_each(|(p, distance)| {
                    let valve_open_time = time + distance + 1;
                    let valve_open_time_left = Self::get_total_time() - valve_open_time;
                    if valve_open_time_left > 0 {
                        let new_valve_state = self.open_valves | Self::get_valve_mask(*p);
                        if new_valve_state != self.open_valves {
                            let mut child = *self;

                            child.open_valves = new_valve_state;
                            child.pressure_released +=
                                Self::open_valve_value(*p, valve_open_time_left, context);

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
        let mut exp = Exploration::new(rooms, ());
        let mut max_pressure_released = 0;

        exp.explore_avoid_worse::<_, _, StackBag<_>>(
            Point2::initial(0, &exp.context),
            |p, rooms, _| {
                let state_potential = p.state_potential_overestimate(rooms);

                if state_potential == 0 {
                    return ExploreSignals::Skip;
                }
                if p.pressure_released + state_potential <= max_pressure_released {
                    return ExploreSignals::Skip;
                }

                if p.pressure_released > max_pressure_released {
                    // println!("New best PRESSURE {} @ {:?}", max_pressure_released, p);
                    max_pressure_released = p.pressure_released;
                }

                ExploreSignals::Explore
            },
            |_p, _n, _rooms, _| true,
        );

        writeln!(writer, "{}", max_pressure_released).unwrap();
    }
}
