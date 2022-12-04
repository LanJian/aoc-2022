use std::str::FromStr;

use anyhow::{anyhow, Error, Result};

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq)]
struct Assignment {
    low: usize,
    high: usize,
}

impl Assignment {
    /// Returns `true` if the assignment fully contains the other assignment. Returns `false`
    /// otherwise.
    fn fully_contains(&self, other: Assignment) -> bool {
        self.low <= other.low && self.high >= other.high
    }

    /// Returns `true` if the assignment overlaps the other assignment. Returns `false` otherwise.
    fn overlaps(&self, other: Assignment) -> bool {
        !(self.low > other.high || other.low > self.high)
    }
}

impl FromStr for Assignment {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (low, high) = s
            .split_once("-")
            .ok_or_else(|| anyhow!("Invalid assignment input"))?;

        Ok(Self {
            low: low.parse::<usize>()?,
            high: high.parse::<usize>()?,
        })
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Assignments {
    first: Assignment,
    second: Assignment,
}

impl Assignments {
    /// Returns `true` if the first assignment fully contains the second assignment, or vice versa.
    /// Returns `false` otherwise.
    fn fully_overlapping(&self) -> bool {
        self.first.fully_contains(self.second) || self.second.fully_contains(self.first)
    }

    /// Returns `true` if the first assignment overlaps the second assignment. Returns `false`
    /// otherwise.
    fn overlapping(&self) -> bool {
        self.first.overlaps(self.second)
    }
}

impl FromStr for Assignments {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s
            .split_once(",")
            .ok_or_else(|| anyhow!("Invalid input: not enough assignments"))?;

        Ok(Self {
            first: Assignment::from_str(left)?,
            second: Assignment::from_str(right)?,
        })
    }
}

pub fn parse_input(lines: &[String]) -> Result<Vec<Assignments>> {
    lines.iter().map(|l| Assignments::from_str(l)).collect()
}

pub fn part_one(parsed: &Vec<Assignments>) -> usize {
    parsed.iter().filter(|&a| a.fully_overlapping()).count()
}

pub fn part_two(parsed: &Vec<Assignments>) -> usize {
    parsed.iter().filter(|&a| a.overlapping()).count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils;

    #[test]
    fn part_one_test() {
        let lines = utils::load_input("inputs/day_04.example").expect("could not load input");
        let parsed = parse_input(&lines).expect("could not parse input");
        assert_eq!(part_one(&parsed), 2);
    }

    #[test]
    fn part_two_test() {
        let lines = utils::load_input("inputs/day_04.example").expect("could not load input");
        let parsed = parse_input(&lines).expect("could not parse input");
        assert_eq!(part_two(&parsed), 4);
    }
}
