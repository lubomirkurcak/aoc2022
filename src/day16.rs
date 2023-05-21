use std::{collections::HashMap, io::prelude::*, io::BufReader, vec};

use lk_math::geometric_traits::IterateNeighboursContext;

pub type RoomId = i32;

#[derive(Debug)]
pub struct Rooms {
    pub collection: HashMap<RoomId, Room>,
    pub room_ids: HashMap<String, RoomId>,
    pub room_names: Vec<String>,
    pub distances: HashMap<(RoomId, RoomId), i32>,
    pub interesting_rooms: Vec<RoomId>,
}

impl IterateNeighboursContext for Rooms {}

impl Rooms {
    // let dist be a |V| × |V| array of minimum distances initialized to ∞ (infinity)
    fn calculate_shortest_paths(&mut self) {
        // for each edge (u, v) do
        //     dist[u][v] ← w(u, v)  // The weight of the edge (u, v)
        // for each vertex v do
        //     dist[v][v] ← 0
        for &x in self.collection.keys() {
            for &y in self.collection.keys() {
                if x == y {
                    self.distances.insert((x, y), 0);
                } else if self.collection.get(&x).unwrap().connections.contains(&y) {
                    self.distances.insert((x, y), 1);
                } else {
                    self.distances.insert((x, y), 999999999);
                }
            }
        }
        // for k from 1 to |V|
        //     for i from 1 to |V|
        //         for j from 1 to |V|
        //             if dist[i][j] > dist[i][k] + dist[k][j]
        //                 dist[i][j] ← dist[i][k] + dist[k][j]
        //             end if
        for &k in self.collection.keys() {
            for &i in self.collection.keys() {
                for &j in self.collection.keys() {
                    if *self.distances.get(&(i, j)).unwrap()
                        > *self.distances.get(&(i, k)).unwrap()
                            + *self.distances.get(&(k, j)).unwrap()
                    {
                        self.distances.insert(
                            (i, j),
                            self.distances.get(&(i, k)).unwrap()
                                + self.distances.get(&(k, j)).unwrap(),
                        );
                    }
                }
            }
        }
    }

    pub fn from_buffer<T>(reader: BufReader<T>) -> Self
    where
        T: std::io::Read,
    {
        let mut rooms_raw = vec![];

        for line in reader.lines().map(|x| x.unwrap()) {
            // writeln!("{}", line);

            let a = line.split("Valve").collect::<Vec<_>>()[1];
            let a = a.split("has flow rate=").collect::<Vec<_>>();
            let valve = a[0].trim().to_string();
            let a = a[1].split(';').collect::<Vec<_>>();
            let pressure = a[0].trim().parse().unwrap();
            let tunnels = a[1]
                .split(',')
                .map(|x| x.split_whitespace().last().unwrap().to_string())
                .collect::<Vec<_>>();

            rooms_raw.push((valve, pressure, tunnels));
        }

        let mut room_names = rooms_raw
            .iter()
            .map(|(name, _, _)| name)
            .cloned()
            .collect::<Vec<_>>();
        room_names.sort();

        let room_ids = room_names
            .iter()
            .cloned()
            .enumerate()
            .map(|(k, v)| (v, k as RoomId))
            .collect::<HashMap<_, _>>();

        let mut rooms = HashMap::new();
        rooms_raw.iter().for_each(|(name, pressure, tunnels)| {
            let room_id = *room_ids.get(name).unwrap();
            rooms.insert(
                room_id,
                Room {
                    pressure: *pressure,
                    connections: tunnels
                        .iter()
                        .map(|x| *room_ids.get(x).unwrap() as RoomId)
                        .collect(),
                    floyd_warshall_connections: vec![],
                },
            );
        });

        let interesting_rooms = rooms
            .iter()
            .filter_map(|(k, v)| if v.pressure > 0 { Some(*k) } else { None })
            .collect::<Vec<_>>();

        let mut rooms = Rooms {
            collection: rooms,
            room_ids,
            room_names,
            distances: HashMap::new(),
            interesting_rooms,
        };

        rooms.calculate_shortest_paths();

        for (k, room) in rooms.collection.iter_mut() {
            for interesting_room in rooms.interesting_rooms.iter() {
                let distance = *rooms.distances.get(&(*k, *interesting_room)).unwrap();
                room.floyd_warshall_connections
                    .push((*interesting_room, distance));
            }
        }

        rooms
    }
}

#[derive(Debug)]
pub struct Room {
    // id: RoomId,
    pub pressure: i32,
    pub connections: Vec<RoomId>,
    pub floyd_warshall_connections: Vec<(RoomId, i32)>,
}

pub trait PointTrait
where
    Self: Sized,
{
    fn get_total_time() -> i32;

    fn initial_valve_state(rooms: &Rooms) -> u64 {
        let mut initial_state = 0;
        for (k, v) in rooms.collection.iter() {
            if v.pressure <= 0 {
                initial_state |= 1u64 << k;
            }
        }

        initial_state
    }

    fn time_left(&self) -> i32;

    fn get_valve_mask(room_id: RoomId) -> u64 {
        1u64 << room_id
    }

    fn open_valve_value(room_id: RoomId, time_left: i32, rooms: &Rooms) -> u64 {
        let room = rooms.collection.get(&room_id).unwrap();
        let pressure: u64 = room.pressure.try_into().unwrap();
        let time_left: u64 = time_left.try_into().unwrap();
        pressure * time_left
    }

    fn get_open_valves(&self) -> u64;

    fn get_releasable_pressures(&self, rooms: &Rooms) -> Vec<i32> {
        rooms
            .collection
            .iter()
            .filter_map(|(k, v)| {
                let mask = 1u64 << k;
                if (self.get_open_valves() & mask) == 0 {
                    return Some(v.pressure);
                }
                None
            })
            .collect()
    }

    fn state_potential_overestimate_v0(&self, rooms: &Rooms) -> u64 {
        let mut potential = 0u64;
        let time_left = self.time_left();
        let time_left = time_left - 1;
        if time_left > 0 {
            let sum: i32 = self.get_releasable_pressures(rooms).iter().sum();
            potential = sum as u64 * time_left as u64;
        }
        potential
    }

    fn state_potential_overestimate_v1(&self, rooms: &Rooms) -> u64 {
        let mut res = 0;
        let mut tl = self.time_left() - 1;
        let mut ps = self.get_releasable_pressures(rooms);
        ps.sort();
        ps.reverse();
        for p in ps.iter() {
            res += tl * p;
            tl -= 1;
        }

        if res > 0 {
            res as u64
        } else {
            0
        }
    }
}
