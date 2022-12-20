use anyhow::{anyhow, Error, Result};
use regex::Regex;
use rustc_hash::{FxHashMap, FxHashSet};
use std::{
    ops::{Add, AddAssign, Sub, SubAssign},
    str::FromStr,
};

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash)]
struct Resources {
    ore: usize,
    clay: usize,
    obsidian: usize,
    geode: usize,
}

impl Resources {
    pub fn new(ore: usize, clay: usize, obsidian: usize, geode: usize) -> Self {
        Self {
            ore,
            clay,
            obsidian,
            geode,
        }
    }
}

impl Add for Resources {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            ore: self.ore + rhs.ore,
            clay: self.clay + rhs.clay,
            obsidian: self.obsidian + rhs.obsidian,
            geode: self.geode + rhs.geode,
        }
    }
}

impl AddAssign for Resources {
    fn add_assign(&mut self, rhs: Self) {
        self.ore += rhs.ore;
        self.clay += rhs.clay;
        self.obsidian += rhs.obsidian;
        self.geode += rhs.geode;
    }
}

impl Sub for Resources {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            ore: self.ore - rhs.ore,
            clay: self.clay - rhs.clay,
            obsidian: self.obsidian - rhs.obsidian,
            geode: self.geode - rhs.geode,
        }
    }
}

impl SubAssign for Resources {
    fn sub_assign(&mut self, rhs: Self) {
        self.ore -= rhs.ore;
        self.clay -= rhs.clay;
        self.obsidian -= rhs.obsidian;
        self.geode -= rhs.geode;
    }
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq)]
struct Robot {
    cost: Resources,
    output: Resources,
}

impl Robot {
    fn build(&self, inventory: &mut Resources, rate: &mut Resources) -> bool {
        if inventory.ore >= self.cost.ore
            && inventory.clay >= self.cost.clay
            && inventory.obsidian >= self.cost.obsidian
            && inventory.geode >= self.cost.geode
        {
            *inventory -= self.cost;
            *rate += self.output;
            return true;
        }

        false
    }

    fn dismantle(&self, inventory: &mut Resources, rate: &mut Resources) {
        *inventory += self.cost;
        *rate -= self.output;
    }
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq)]
struct Blueprint {
    id: usize,
    robots: [Robot; 4],
}

impl FromStr for Blueprint {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(
            "Blueprint (\\d+): \
            Each ore robot costs (\\d+) ore. \
            Each clay robot costs (\\d+) ore. \
            Each obsidian robot costs (\\d+) ore and (\\d+) clay. \
            Each geode robot costs (\\d+) ore and (\\d+) obsidian.",
        )?;

        let caps = re.captures(s).ok_or_else(|| anyhow!("Invalid input"))?;

        Ok(Self {
            id: caps[1].parse()?,
            robots: [
                Robot {
                    cost: Resources::new(caps[2].parse()?, 0, 0, 0),
                    output: Resources::new(1, 0, 0, 0),
                },
                Robot {
                    cost: Resources::new(caps[3].parse()?, 0, 0, 0),
                    output: Resources::new(0, 1, 0, 0),
                },
                Robot {
                    cost: Resources::new(caps[4].parse()?, caps[5].parse()?, 0, 0),
                    output: Resources::new(0, 0, 1, 0),
                },
                Robot {
                    cost: Resources::new(caps[6].parse()?, 0, caps[7].parse()?, 0),
                    output: Resources::new(0, 0, 0, 1),
                },
            ],
        })
    }
}

#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct Factory {
    blueprints: Vec<Blueprint>,
}

impl Factory {
    fn simulate(&self) -> usize {
        let mut sum = 0;

        for blueprint in &self.blueprints {
            sum += blueprint.id * self.dfs(
                blueprint,
                10,
                &mut Resources::default(),
                &mut Resources::new(1, 0, 0, 0),
                &mut FxHashMap::default(),
            )
        }

        sum
    }

    fn simulate_2(&self) -> usize {
        let len = self.blueprints.len();
        let mut product = 1;

        for blueprint in &self.blueprints[0..3.min(len)] {
            product *= self.dfs(
                blueprint,
                32,
                &mut Resources::default(),
                &mut Resources::new(1, 0, 0, 0),
                &mut FxHashMap::default(),
            )
        }

        product
    }

    fn dfs(
        &self,
        blueprint: &Blueprint,
        remaining: usize,
        inventory: &mut Resources,
        rate: &mut Resources,
        seen: &mut FxHashMap<(usize, Resources, Resources), usize>,
    ) -> usize {
        //dbg!(&remaining);
        //dbg!(&inventory);

        if remaining == 0 {
            return inventory.geode;
        }

        if let Some(answer) = seen.get(&(remaining, *inventory, *rate)) {
            return *answer;
        }

        let mut max = 0;

        *inventory += *rate;
        max = max.max(self.dfs(blueprint, remaining - 1, inventory, rate, seen));
        *inventory -= *rate;

        for robot in blueprint.robots {
            let old_rate = *rate;
            if robot.build(inventory, rate) {
                *inventory += old_rate;
                max = max.max(self.dfs(blueprint, remaining - 1, inventory, rate, seen));
                *inventory -= old_rate;

                robot.dismantle(inventory, rate);
            }
        }

        seen.insert((remaining, *inventory, *rate), max);

        max
    }
}

impl TryFrom<&[String]> for Factory {
    type Error = Error;

    fn try_from(lines: &[String]) -> Result<Self, Self::Error> {
        Ok(Factory {
            blueprints: lines
                .iter()
                .map(|l| Blueprint::from_str(l))
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

pub fn parse_input(lines: &[String]) -> Result<Factory> {
    Factory::try_from(lines)
}

pub fn part_one(parsed: &Factory) -> usize {
    parsed.simulate()
}

pub fn part_two(parsed: &Factory) -> usize {
    parsed.simulate_2()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils;

    #[test]
    fn part_one_test() {
        let lines = utils::load_input("inputs/day_19.example").expect("could not load input");
        let parsed = parse_input(&lines).expect("could not parse input");
        assert_eq!(part_one(&parsed), 33);
    }

    #[test]
    fn part_two_test() {
        let lines = utils::load_input("inputs/day_19.example").expect("could not load input");
        let parsed = parse_input(&lines).expect("could not parse input");
        assert_eq!(part_two(&parsed), 3472);
    }
}
