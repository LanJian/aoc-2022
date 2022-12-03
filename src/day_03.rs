use std::str::FromStr;

use anyhow::{anyhow, Error, Result};
use rustc_hash::FxHashMap;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash)]
struct Item {
    kind: char,
}

impl Item {
    fn priority(&self) -> usize {
        if self.kind.is_lowercase() {
            self.kind as usize - 'a' as usize + 1
        } else {
            self.kind as usize - 'A' as usize + 27
        }
    }
}

impl From<char> for Item {
    fn from(c: char) -> Self {
        Self { kind: c }
    }
}

#[derive(Debug, Clone, Default)]
pub struct RuckSack {
    first_compartment: FxHashMap<Item, usize>,
    second_compartment: FxHashMap<Item, usize>,
}

impl RuckSack {
    /// Returns the [Item] that appears in both compartments. It is assumed that there can be at
    /// most 1 such item.
    fn common_item(&self) -> Option<&Item> {
        self.first_compartment
            .keys()
            .find(|&item| self.second_compartment.contains_key(item))
    }

    /// Returns the [Item] that appears in all 3 [RuckSack]s. It is assume that there can be at
    /// most 1 such item.
    fn common_item_in_group(&self, second: &RuckSack, third: &RuckSack) -> Option<&Item> {
        self.first_compartment
            .keys()
            .find(|&item| {
                (second.first_compartment.contains_key(item)
                    || second.second_compartment.contains_key(item))
                    && (third.first_compartment.contains_key(item)
                        || third.second_compartment.contains_key(item))
            })
            .or_else(|| {
                self.second_compartment.keys().find(|&item| {
                    (second.first_compartment.contains_key(item)
                        || second.second_compartment.contains_key(item))
                        && (third.first_compartment.contains_key(item)
                            || third.second_compartment.contains_key(item))
                })
            })
    }
}

impl FromStr for RuckSack {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let len = s.len();
        let mut chars = s.chars();
        let mut first_compartment = FxHashMap::default();
        let mut second_compartment = FxHashMap::default();

        for _ in 0..len / 2 {
            let c = chars
                .next()
                .ok_or_else(|| anyhow!("Not enough items in rucksack"))?;
            first_compartment
                .entry(c.into())
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }

        for _ in len / 2..len {
            let c = chars
                .next()
                .ok_or_else(|| anyhow!("Not enough items in rucksack"))?;
            second_compartment
                .entry(c.into())
                .and_modify(|count| *count += 1)
                .or_insert(1);
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
        .map(|rucksack| {
            rucksack
                .common_item()
                .map(|item| item.priority())
                .unwrap_or(0)
        })
        .sum()
}

pub fn part_two(parsed: &Vec<RuckSack>) -> usize {
    parsed
        .chunks(3)
        .map(|rucksacks| {
            rucksacks[0]
                .common_item_in_group(&rucksacks[1], &rucksacks[2])
                .map(|item| item.priority())
                .unwrap_or(0)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::utils;

    use super::*;

    #[test]
    fn priority_test() {
        assert_eq!(Item::from('a').priority(), 1);
        assert_eq!(Item::from('z').priority(), 26);
        assert_eq!(Item::from('A').priority(), 27);
        assert_eq!(Item::from('Z').priority(), 52);
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
