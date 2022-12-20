use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt::Display,
    hash::Hash,
    io::prelude::*,
    io::BufReader,
    vec,
};

use crate::{lkc::geometric_traits::IterateNeighbours, Day, Problem};

type RoomId = i32;

#[derive(Debug)]
struct Rooms {
    collection: HashMap<RoomId, Room>,
    room_ids: HashMap<String, RoomId>,
    room_names: Vec<String>,
    distances: HashMap<(RoomId, RoomId), i32>,
}

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

    fn from_buffer<T>(reader: BufReader<T>) -> Self
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
                },
            );
        });

        let mut rooms = Rooms {
            collection: rooms,
            room_ids,
            room_names,
            distances: HashMap::new(),
        };

        rooms.calculate_shortest_paths();

        rooms
    }
}

#[derive(Debug)]
struct Room {
    // id: RoomId,
    pressure: i32,
    connections: Vec<RoomId>,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Point {
    room_id: RoomId,
    time: i32,
    open_valves: u64,
    pressure_released: u64,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Point2 {
    me_p: RoomId,
    elephant_p: RoomId,
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

trait PointTrait
where
    Self: Sized,
{
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
}

impl PointTrait for Point {
    fn time_left(&self) -> i32 {
        30 - self.time
    }
}

impl PointTrait for Point2 {
    fn time_left(&self) -> i32 {
        26 - self.time
    }
}

impl Point2 {
    pub fn initial(room_id: RoomId, rooms: &Rooms) -> Self {
        Self {
            me_p: room_id,
            elephant_p: room_id,
            time: 0,
            open_valves: Self::initial_valve_state(rooms),
            pressure_released: 0,
        }
    }

    pub fn new(
        me_p: RoomId,
        elephant_p: RoomId,
        time: i32,
        open_valves: u64,
        pressure_released: u64,
    ) -> Self {
        Self {
            me_p,
            elephant_p,
            time,
            open_valves,
            pressure_released,
        }
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

    pub fn new(room_id: RoomId, time: i32, open_valves: u64, pressure_released: u64) -> Self {
        Self {
            room_id,
            time,
            open_valves,
            pressure_released,
        }
    }

    pub fn open_valve_value(&self, rooms: &Rooms) -> u64 {
        if self.time_left() < 2 {
            return 0;
        }

        let room = rooms.collection.get(&self.room_id).unwrap();
        (room.pressure * (self.time_left() - 1)).try_into().unwrap()
    }

    pub fn state_potential_overestimate(&self, rooms: &Rooms) -> u64 {
        let mut potential = 0u64;
        let time_left = self.time_left();
        for (k, v) in rooms.collection.iter() {
            let mask = 1u64 << k;
            if (self.open_valves & mask) == 0 {
                potential += (v.pressure * time_left) as u64;
            }
        }
        potential
    }

    pub fn is_open(&self) -> bool {
        self.open_valves & Self::get_valve_mask(self.room_id) > 0
    }

    pub fn open_valve(&self, rooms: &Rooms) -> Option<Self> {
        if !self.is_open() {
            let potential = self.open_valve_value(rooms);
            if potential > 0 {
                return Some(Self::new(
                    self.room_id,
                    self.time + 1,
                    self.open_valves | Self::get_valve_mask(self.room_id),
                    self.pressure_released + potential,
                ));
            }
        }
        None
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
                .connections
                .iter()
                .for_each(|p| {
                    options.push(Self::new(
                        *p,
                        self.time + 1,
                        self.open_valves,
                        self.pressure_released,
                    ))
                });

            if let Some(open) = self.open_valve(rooms) {
                options.push(open);
            }
        }

        options
    }
}

#[derive(Debug)]
struct Exploration<P: IterateNeighbours, S> {
    structure: S,
    phantom: std::marker::PhantomData<P>,
}

pub trait Bag<T> {
    fn new() -> Self;
    fn put(&mut self, t: T);
    fn get(&mut self) -> Option<T>;
    fn len(&self) -> usize;

    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

struct StackBag<T>(Vec<T>);
struct QueueBag<T>(VecDeque<T>);

impl<T> Bag<T> for StackBag<T> {
    fn put(&mut self, t: T) {
        self.0.push(t)
    }
    fn get(&mut self) -> Option<T> {
        self.0.pop()
    }
    fn len(&self) -> usize {
        self.0.len()
    }
    fn new() -> Self {
        Self(vec![])
    }
}
impl<T> Bag<T> for QueueBag<T> {
    fn put(&mut self, t: T) {
        self.0.push_back(t)
    }
    fn get(&mut self) -> Option<T> {
        self.0.pop_front()
    }
    fn len(&self) -> usize {
        self.0.len()
    }
    fn new() -> Self {
        Self(VecDeque::new())
    }
}

enum ExploreSignals {
    FoundGoal,
    Explore,
    Skip,
}

impl<P: Clone + Copy> Exploration<P, Rooms>
where
    P: IterateNeighbours<Context = Self> + Hash + Eq,
{
    pub fn new(structure: Rooms) -> Self {
        Self {
            structure,
            phantom: std::marker::PhantomData,
        }
    }

    pub fn explore<F, G>(&self, start: P, mut goal: G, mut filter_neighbours: F)
    where
        F: FnMut(&P) -> bool,
        G: FnMut(&P) -> ExploreSignals,
    {
        self.explore_advanced(
            start,
            HashSet::new(),
            |p, data| {
                data.insert(*p);
                goal(p)
            },
            |p, data| !data.contains(p) && filter_neighbours(p),
        )
    }

    pub fn explore_advanced<T, F, G>(
        &self,
        start: P,
        mut data: T,
        mut goal: G,
        mut filter_neighbours: F,
    ) where
        F: FnMut(&P, &mut T) -> bool,
        G: FnMut(&P, &mut T) -> ExploreSignals,
    {
        let mut open = StackBag::new();
        open.put(start);
        while !open.is_empty() {
            let p = open.get().unwrap();

            match goal(&p, &mut data) {
                ExploreSignals::FoundGoal => break,
                ExploreSignals::Explore => (),
                ExploreSignals::Skip => continue,
            }

            for neighbour in p.neighbours(self) {
                if filter_neighbours(&neighbour, &mut data) {
                    open.put(neighbour);
                }
            }
        }
    }
}

impl Problem for Day<16> {
    fn solve_buffer<T, W>(reader: BufReader<T>, writer: &mut W)
    where
        T: std::io::Read,
        W: std::io::Write,
    {
        let rooms = Rooms::from_buffer(reader);
        let exp = Exploration::new(rooms);
        let mut max_pressure_released = 0;

        exp.explore_advanced(
            Point::initial(0, &exp.structure),
            HashMap::new(),
            |p, data| {
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

                let k = (p.open_valves, p.room_id);

                if let Some(&(t0, s0)) = data.get(&k) {
                    if p.time >= t0 && p.pressure_released <= s0 {
                        return ExploreSignals::Skip;
                    }

                    if p.time <= t0 && p.pressure_released >= s0 {
                        data.insert(k, (p.time, p.pressure_released));
                    }
                } else {
                    data.insert(k, (p.time, p.pressure_released));
                }

                ExploreSignals::Explore
            },
            |p, data| {
                let k = (p.open_valves, p.room_id);

                if let Some(&(t0, s0)) = data.get(&k) {
                    if p.time >= t0 && p.pressure_released <= s0 {
                        return false;
                    }
                }

                true
            },
        );

        println!("MAX PRESSURE {}", max_pressure_released);

        writeln!(writer, "Result: {}", max_pressure_released).unwrap();
    }
}
