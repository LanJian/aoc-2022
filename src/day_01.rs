use std::num::ParseIntError;

use anyhow::Result;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, PartialOrd, Ord)]
pub struct Elf {
    calories: usize,
}

impl TryFrom<&[String]> for Elf {
    type Error = ParseIntError;

    fn try_from(value: &[String]) -> Result<Self, Self::Error> {
        let calories: Result<Vec<usize>, _> = value.iter().map(|v| v.parse::<usize>()).collect();
        Ok(Self {
            calories: calories?.iter().sum(),
        })
    }
}

pub fn parse_input(lines: Vec<String>) -> Result<Vec<Elf>> {
    let elves = lines
        .split(|line| line == "")
        .map(|group| Elf::try_from(group))
        .collect::<Result<Vec<_>, _>>()?;

    Ok(elves)
}

pub fn part_one(parsed: &Vec<Elf>) -> usize {
    parsed.iter().max().map(|elf| elf.calories).unwrap_or(0)
}

pub fn part_two(parsed: &Vec<Elf>) -> usize {
    let mut a = usize::MIN;
    let mut b = usize::MIN;
    let mut c = usize::MIN;

    for elf in parsed {
        if elf.calories >= a {
            c = b;
            b = a;
            a = elf.calories;
        } else if elf.calories >= b {
            c = b;
            b = elf.calories;
        } else if elf.calories > c {
            c = elf.calories
        }
    }

    a + b + c
}

#[cfg(test)]
mod tests {
    use crate::utils;

    use super::*;

    #[test]
    fn part_one_test() {
        let lines = utils::load_input("inputs/day_01.example").expect("could not load input");
        let parsed = parse_input(lines).expect("could not parse input");
        assert_eq!(part_one(&parsed), 24000);
    }

    #[test]
    fn part_two_test() {
        let lines = utils::load_input("inputs/day_01.example").expect("could not load input");
        let parsed = parse_input(lines).expect("could not parse input");
        assert_eq!(part_two(&parsed), 45000);
    }
}
