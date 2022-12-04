use std::str::FromStr;

use anyhow::{anyhow, Error, Result};

fn char_to_priority(c: &char) -> usize {
    if c.is_lowercase() {
        *c as usize - 'a' as usize + 1
    } else {
        *c as usize - 'A' as usize + 27
    }
}

fn char_to_bitmask(c: &char) -> u64 {
    1 << char_to_priority(c)
}

#[derive(Debug, Clone, Default)]
pub struct RuckSack {
    first_compartment: u64,
    second_compartment: u64,
}

impl RuckSack {
    /// Returns the priority of the item that appears in both compartments. It is assumed that there
    /// can be at most 1 such item.
    fn common_item_priority(&self) -> Option<usize> {
        let combined = self.first_compartment & self.second_compartment;
        (1..=52).find(|x| combined >> x == 1)
    }

    /// Returns the priority of the item that appears in all 3 [RuckSack]s. It is assume that there
    /// can be at most 1 such item.
    fn common_item_priority_in_group(&self, second: &RuckSack, third: &RuckSack) -> Option<usize> {
        let combined = self.union() & second.union() & third.union();
        (1..=52).find(|x| combined >> x == 1)
    }

    fn union(&self) -> u64 {
        self.first_compartment | self.second_compartment
    }
}

impl FromStr for RuckSack {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let len = s.len();
        let mut chars = s.chars();
        let mut first_compartment = 0;
        let mut second_compartment = 0;

        for _ in 0..len / 2 {
            let c = chars
                .next()
                .ok_or_else(|| anyhow!("Not enough items in rucksack"))?;
            let p = char_to_bitmask(&c);
            first_compartment |= p;
        }

        for _ in len / 2..len {
            let c = chars
                .next()
                .ok_or_else(|| anyhow!("Not enough items in rucksack"))?;
            let p = char_to_bitmask(&c);
            second_compartment |= p;
        }

        Ok(RuckSack {
            first_compartment,
            second_compartment,
        })
    }
}

pub fn parse_input(lines: &[String]) -> Result<Vec<RuckSack>> {
    lines.iter().map(|l| RuckSack::from_str(l)).collect()
}

pub fn part_one(parsed: &Vec<RuckSack>) -> usize {
    parsed
        .iter()
        .map(|rucksack| rucksack.common_item_priority().unwrap_or(0))
        .sum()
}

pub fn part_two(parsed: &Vec<RuckSack>) -> usize {
    parsed
        .chunks(3)
        .map(|rucksacks| {
            rucksacks[0]
                .common_item_priority_in_group(&rucksacks[1], &rucksacks[2])
                .unwrap_or(0)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::utils;

    use super::*;

    #[test]
    fn char_to_priority_test() {
        assert_eq!(char_to_priority(&'a'), 1);
        assert_eq!(char_to_priority(&'z'), 26);
        assert_eq!(char_to_priority(&'A'), 27);
        assert_eq!(char_to_priority(&'Z'), 52);
    }

    #[test]
    fn part_one_test() {
        let lines = utils::load_input("inputs/day_03.example").expect("could not load input");
        let parsed = parse_input(&lines).expect("could not parse input");
        assert_eq!(part_one(&parsed), 157);
    }

    #[test]
    fn part_two_test() {
        let lines = utils::load_input("inputs/day_03.example").expect("could not load input");
        let parsed = parse_input(&lines).expect("could not parse input");
        assert_eq!(part_two(&parsed), 70);
    }
}
