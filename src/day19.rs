use std::{io::prelude::*, io::BufReader, str::FromStr};

use crate::{
    lkc::{
        explore::{Exploration, ExploreSignals},
        geometric_traits::{IterateNeighbours, IterateNeighboursContext}, sketch::StackBag,
    },
    Problem,
};

type Ore = i32;
type Clay = i32;
type Obsidian = i32;
type Geode = i32;

struct Blueprint {
    id: i32,
    ore_r_cost: Ore,
    clay_r_cost: Ore,
    obs_r_cost_ore: Ore,
    obs_r_cost_clay: Clay,
    geode_r_cost_ore: Ore,
    geode_r_cost_obs: Obsidian,
}

impl FromStr for Blueprint {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_a, s) = s.split_once("Blueprint").ok_or(())?;
        let (a, s) = s.split_once(": Each ore robot costs").ok_or(())?;
        let id = a.trim().parse().or(Err(()))?;
        let (a, s) = s.split_once("ore. Each clay robot costs").ok_or(())?;
        let ore_r_cost = a.trim().parse().or(Err(()))?;
        let (a, s) = s.split_once("ore. Each obsidian robot costs").ok_or(())?;
        let clay_r_cost = a.trim().parse().or(Err(()))?;
        let (a, s) = s.split_once("ore and").ok_or(())?;
        let obs_r_cost_ore = a.trim().parse().or(Err(()))?;
        let (a, s) = s.split_once("clay. Each geode robot costs").ok_or(())?;
        let obs_r_cost_clay = a.trim().parse().or(Err(()))?;
        let (a, s) = s.split_once("ore and").ok_or(())?;
        let geode_r_cost_ore = a.trim().parse().or(Err(()))?;
        let (a, _s) = s.split_once("obsidian.").ok_or(())?;
        let geode_r_cost_obs = a.trim().parse().or(Err(()))?;

        Ok(Self {
            id,
            ore_r_cost,
            clay_r_cost,
            obs_r_cost_ore,
            obs_r_cost_clay,
            geode_r_cost_ore,
            geode_r_cost_obs,
        })
    }
}
impl IterateNeighboursContext for Blueprint {}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Clone, Copy)]
struct Point<const C: i32> {
    time: i32,

    ore: Ore,
    clay: Clay,
    obs: Obsidian,
    geode: Geode,

    ore_r: i32,
    clay_r: i32,
    obs_r: i32,
    geode_r: i32,
}

impl<const C: i32> Point<C> {
    fn wait(&self) -> Self {
        let mut result = *self;

        result.time += 1;
        result.ore += result.ore_r;
        result.clay += result.clay_r;
        result.obs += result.obs_r;
        result.geode += result.geode_r;

        result
    }

    fn can_buy_orebots(&self, bp: &Blueprint) -> i32 {
        self.ore / bp.ore_r_cost
    }
    fn can_buy_claybots(&self, bp: &Blueprint) -> i32 {
        self.ore / bp.clay_r_cost
    }
    fn can_buy_obsibots(&self, bp: &Blueprint) -> i32 {
        std::cmp::min(self.ore / bp.obs_r_cost_ore, self.clay / bp.obs_r_cost_clay)
    }
    fn can_buy_geobots(&self, bp: &Blueprint) -> i32 {
        std::cmp::min(
            self.ore / bp.geode_r_cost_ore,
            self.obs / bp.geode_r_cost_obs,
        )
    }

    fn can_buy_orebot(&self, bp: &Blueprint) -> bool {
        self.ore >= bp.ore_r_cost
    }
    fn can_buy_claybot(&self, bp: &Blueprint) -> bool {
        self.ore >= bp.clay_r_cost
    }
    fn can_buy_obsibot(&self, bp: &Blueprint) -> bool {
        self.ore >= bp.obs_r_cost_ore && self.clay >= bp.obs_r_cost_clay
    }
    fn can_buy_geobot(&self, bp: &Blueprint) -> bool {
        self.ore >= bp.geode_r_cost_ore && self.obs >= bp.geode_r_cost_obs
    }

    fn buy_orebot(&self, bp: &Blueprint) -> Option<Self> {
        if self.can_buy_orebot(bp) {
            let mut result = self.wait();
            result.ore -= bp.ore_r_cost;
            result.ore_r += 1;
            Some(result)
        } else {
            None
        }
    }
    fn buy_claybot(&self, bp: &Blueprint) -> Option<Self> {
        if self.can_buy_claybot(bp) {
            let mut result = self.wait();
            result.ore -= bp.clay_r_cost;
            result.clay_r += 1;
            Some(result)
        } else {
            None
        }
    }
    fn buy_obsibot(&self, bp: &Blueprint) -> Option<Self> {
        if self.can_buy_obsibot(bp) {
            let mut result = self.wait();
            result.ore -= bp.obs_r_cost_ore;
            result.clay -= bp.obs_r_cost_clay;
            result.obs_r += 1;
            Some(result)
        } else {
            None
        }
    }
    fn buy_geobot(&self, bp: &Blueprint) -> Option<Self> {
        if self.can_buy_geobot(bp) {
            let mut result = self.wait();
            result.ore -= bp.geode_r_cost_ore;
            result.obs -= bp.geode_r_cost_obs;
            result.geode_r += 1;
            Some(result)
        } else {
            None
        }
    }
}

impl<const C: i32> IterateNeighbours<Blueprint> for Point<C> {
    fn neighbours(&self, bp: &Blueprint) -> Vec<Self> {
        if self.time >= C {
            return vec![];
        }
        if self.time >= C - 1 {
            // NOTE(lubo): Even if we started building a geobot, it wouldn't have time to produce any geodes.
            return vec![self.wait()];
        }
        if self.time >= C - 2 {
            // NOTE(lubo): Last change to build a geobot that will produce 1 geode.
            if self.can_buy_geobot(bp) {
                return vec![self.buy_geobot(bp).unwrap().wait()];
            } else {
                return vec![self.wait().wait()];
            }
        }

        let mut working = 0b1111;
        let mut results = [None; 4];

        let time_left = C - self.time;
        let bots = self.can_buy_orebots(bp);
        if bots >= time_left {
            working &= 0b1110;
        }
        let bots = self.can_buy_claybots(bp);
        if bots >= time_left {
            working &= 0b1101;
        }
        let bots = self.can_buy_obsibots(bp);
        if bots >= time_left {
            working &= 0b1011;
        }
        let bots = self.can_buy_geobots(bp);
        if bots >= time_left {
            working &= 0b0111;
        }

        let mut probe = *self;

        for _time in self.time..C {
            if (working & 0b0001) > 0 && probe.can_buy_orebot(bp) {
                working &= 0b1110;
                results[0] = Some(probe.buy_orebot(bp).unwrap());
            }
            if (working & 0b0010) > 0 && probe.can_buy_claybot(bp) {
                working &= 0b1101;
                results[1] = Some(probe.buy_claybot(bp).unwrap());
            }
            if (working & 0b0100) > 0 && probe.can_buy_obsibot(bp) {
                working &= 0b1011;
                results[2] = Some(probe.buy_obsibot(bp).unwrap());
            }
            if (working & 0b1000) > 0 && probe.can_buy_geobot(bp) {
                working &= 0b0111;
                results[3] = Some(probe.buy_geobot(bp).unwrap());
            }
            if working == 0 {
                break;
            }

            probe = probe.wait();
        }

        let time_left = C - self.time;
        if results[0].is_some() {
            let bots = results[0].unwrap().can_buy_orebots(bp);
            if bots >= time_left {
                results[0] = None;
            }
        }
        if results[1].is_some() {
            let bots = results[1].unwrap().can_buy_claybots(bp);
            if bots >= time_left {
                results[1] = None;
            }
        }
        if results[2].is_some() {
            let bots = results[2].unwrap().can_buy_obsibots(bp);
            if bots >= time_left {
                results[2] = None;
            }
        }
        if results[3].is_some() {
            let bots = results[3].unwrap().can_buy_geobots(bp);
            if bots >= time_left {
                results[3] = None;
            }
        }

        if results[0].is_none()
            && results[1].is_none()
            && results[2].is_none()
            && results[3].is_none()
        {
            let mut a = *self;
            while a.time < C {
                a = a.wait();
            }
            return vec![a];
        }

        results
            .into_iter()
            .flatten()
            .filter(|x| x.time <= C)
            .collect::<Vec<_>>()
    }
}

pub struct Day19<const C: i32, const B: bool>;

impl<const C: i32, const B: bool> Problem for Day19<C, B> {
    fn solve_buffer<T, W>(reader: BufReader<T>, writer: &mut W)
    where
        T: std::io::Read,
        W: std::io::Write,
    {
        let lines = reader.lines().map(|x| x.unwrap()).collect::<Vec<_>>();
        let blueprints = lines
            .iter()
            .map(|x| Blueprint::from_str(x).unwrap())
            .collect::<Vec<_>>();

        let blueprints = if B {
            blueprints.into_iter().take(3).collect()
        } else {
            blueprints
        };

        let mut result = i32::from(B);

        for blueprint in blueprints {
            let mut exp = Exploration::new(blueprint, ());
            let mut max_geodes_for_bp = 0;
            exp.explore::<_, _, StackBag<_>>(
                Point::<C> {
                    ore_r: 1,
                    ..Default::default()
                },
                |p, _bp, _| {
                    if p.geode > max_geodes_for_bp {
                        max_geodes_for_bp = p.geode;
                        println!("new best {:?}", p);
                    }

                    ExploreSignals::Explore
                },
                |_p, _n, _bp, _| true,
            );
            if B {
                result *= max_geodes_for_bp;
            } else {
                result += exp.context.id * max_geodes_for_bp;
            }
            println!(
                "Bp {} collected {} geodes.   Result so far: {}",
                exp.context.id, max_geodes_for_bp, result
            );
        }

        writeln!(writer, "{}", result).unwrap();
    }
}
