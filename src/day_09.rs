use anyhow::{anyhow, bail, Error, Result};
use rustc_hash::FxHashSet;
use std::str::FromStr;

use crate::grid::Coordinate;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl FromStr for Direction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "U" => Self::Up,
            "D" => Self::Down,
            "L" => Self::Left,
            "R" => Self::Right,
            _ => bail!("Invalid direction: {}", s),
        })
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Motion {
    dir: Direction,
    len: usize,
}

impl FromStr for Motion {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, b) = s
            .split_once(" ")
            .ok_or_else(|| anyhow!("Invalid motion: {}", s))?;

        Ok(Self {
            dir: Direction::from_str(a)?,
            len: b.parse()?,
        })
    }
}

#[derive(Debug, Clone, Default, Eq, PartialEq)]
struct State {
    rope: Vec<Coordinate>,
    visited: FxHashSet<Coordinate>,
}

impl State {
    fn new(len: usize) -> Self {
        Self {
            rope: vec![Coordinate::default(); len],
            visited: FxHashSet::default(),
        }
    }

    fn simulate(&mut self, motions: &[Motion]) {
        if self.rope.len() < 2 {
            // there's no tail, so nothing to simulate
            return;
        }

        for motion in motions {
            for _ in 0..motion.len {
                self.simulate_step(&motion.dir)
            }
        }
    }

    fn simulate_step(&mut self, dir: &Direction) {
        self.rope[0] = match dir {
            Direction::Up => self.rope[0].north(),
            Direction::Down => self.rope[0].south(),
            Direction::Left => self.rope[0].west(),
            Direction::Right => self.rope[0].east(),
        };

        for i in 1..self.rope.len() {
            self.rope[i] = match (
                self.rope[i].x() - self.rope[i - 1].x(),
                self.rope[i].y() - self.rope[i - 1].y(),
            ) {
                (dx, dy) if dx.abs() < 2 && dy.abs() < 2 => self.rope[i],
                (0, dy) if dy > 1 => self.rope[i].south(),
                (0, dy) if dy < -1 => self.rope[i].north(),
                (dx, 0) if dx > 1 => self.rope[i].west(),
                (dx, 0) if dx < -1 => self.rope[i].east(),
                (dx, dy) if (dx > 1 && dy > 0) || (dx > 0 && dy > 1) => self.rope[i].southwest(),
                (dx, dy) if (dx > 1 && dy < 0) || (dx > 0 && dy < -1) => self.rope[i].northwest(),
                (dx, dy) if (dx < -1 && dy > 0) || (dx < 0 && dy > 1) => self.rope[i].southeast(),
                (dx, dy) if (dx < -1 && dy < 0) || (dx < 0 && dy < -1) => self.rope[i].northeast(),
                _ => self.rope[i],
            };
        }

        if let Some(&c) = self.rope.last() {
            self.visited.insert(c);
        }
    }

    fn visited_positions(&self) -> usize {
        self.visited.len()
    }
}

pub fn parse_input(lines: &[String]) -> Result<Vec<Motion>> {
    lines.iter().map(|l| Motion::from_str(l)).collect()
}

pub fn part_one(parsed: &Vec<Motion>) -> usize {
    let mut state = State::new(2);
    state.simulate(parsed);
    state.visited_positions()
}

pub fn part_two(parsed: &Vec<Motion>) -> usize {
    let mut state = State::new(10);
    state.simulate(parsed);
    state.visited_positions()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils;

    #[test]
    fn part_one_test() {
        let lines = utils::load_input("inputs/day_09.example").expect("could not load input");
        let parsed = parse_input(&lines).expect("could not parse input");
        assert_eq!(part_one(&parsed), 13);
    }

    #[test]
    fn part_two_test() {
        let lines = utils::load_input("inputs/day_09.example").expect("could not load input");
        let parsed = parse_input(&lines).expect("could not parse input");
        assert_eq!(part_two(&parsed), 1);
    }
}
