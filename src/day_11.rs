use std::collections::VecDeque;
use std::str::FromStr;

use anyhow::{anyhow, bail, Error, Result};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Operation {
    Add(u64),
    Multiply(u64),
    Square,
}

impl FromStr for Operation {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // it looks like:
        //   Operation: new = old + 6
        let tokens = s[23..]
            .split_once(' ')
            .ok_or_else(|| anyhow!("Could not parse operation: {}", s))?;

        let ret = match tokens {
            (_, "old") => Self::Square,
            ("+", value) => Self::Add(value.parse()?),
            ("*", value) => Self::Multiply(value.parse()?),
            _ => bail!("Could not parse operation: {}", s),
        };

        Ok(ret)
    }
}

impl Operation {
    fn apply_to(&self, value: u64) -> u64 {
        match self {
            Self::Add(operand) => value + operand,
            Self::Multiply(operand) => value * operand,
            Self::Square => value * value,
        }
    }
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq)]
struct Test {
    divisible_by: u64,
    true_target: usize,
    false_target: usize,
}

impl TryFrom<&[String]> for Test {
    type Error = Error;

    fn try_from(lines: &[String]) -> Result<Self, Self::Error> {
        // it looks like:
        //   Test: divisible by 19
        //     If true: throw to monkey 2
        //     If false: throw to monkey 0
        if lines.len() != 3 {
            bail!("Test should have 3 lines")
        }

        Ok(Self {
            divisible_by: lines[0][21..].parse()?,
            true_target: lines[1][29..].parse()?,
            false_target: lines[2][30..].parse()?,
        })
    }
}

impl Test {
    /// Performs the test on the given [value] and returns the intended target
    fn apply_to(&self, value: u64) -> usize {
        if value % self.divisible_by == 0 {
            self.true_target
        } else {
            self.false_target
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Monkey {
    items: VecDeque<u64>,
    operation: Operation,
    test: Test,
    num_inspections: usize,
}

impl TryFrom<&[String]> for Monkey {
    type Error = Error;

    fn try_from(lines: &[String]) -> Result<Self, Self::Error> {
        // it looks like:
        // Monkey 0:
        //   Starting items: 79, 98
        //   Operation: new = old * 19
        //   Test: divisible by 23
        //     If true: throw to monkey 2
        //     If false: throw to monkey 3
        if lines.len() != 6 {
            bail!("Monkey should have 6 lines")
        }

        Ok(Self {
            items: lines[1][18..]
                .split(", ")
                .map(|t| t.parse::<u64>())
                .collect::<Result<VecDeque<_>, _>>()?,
            operation: Operation::from_str(&lines[2])?,
            test: lines[3..6].try_into()?,
            num_inspections: 0,
        })
    }
}

fn simulate(monkeys: &mut Vec<Monkey>, rounds: usize, reducer: impl Fn(u64) -> u64) {
    for _ in 0..rounds {
        simulate_one_round(monkeys, &reducer);
    }
}

fn simulate_one_round(monkeys: &mut Vec<Monkey>, reducer: impl Fn(u64) -> u64) {
    for i in 0..monkeys.len() {
        monkeys[i].num_inspections += monkeys[i].items.len();
        while let Some(cur_item) = monkeys[i].items.pop_front() {
            let new_item = reducer(monkeys[i].operation.apply_to(cur_item));
            let target = monkeys[i].test.apply_to(new_item);
            monkeys[target].items.push_back(new_item);
        }
    }
}

pub fn parse_input(lines: &[String]) -> Result<Vec<Monkey>> {
    lines
        .split(|l| l.is_empty())
        .map(|chunk| Monkey::try_from(chunk))
        .collect()
}

pub fn part_one(parsed: &Vec<Monkey>) -> usize {
    let mut monkeys = parsed.clone();
    let len = monkeys.len();
    simulate(&mut monkeys, 20, |x| x / 3);
    monkeys.sort_by_key(|m| m.num_inspections);
    monkeys[len - 1].num_inspections * monkeys[len - 2].num_inspections
}

pub fn part_two(parsed: &Vec<Monkey>) -> usize {
    let mut monkeys = parsed.clone();
    let len = monkeys.len();
    let divisor: u64 = monkeys.iter().map(|m| m.test.divisible_by).product();

    simulate(&mut monkeys, 10000, |x| x % divisor);
    monkeys.sort_by_key(|m| m.num_inspections);
    monkeys[len - 1].num_inspections * monkeys[len - 2].num_inspections
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils;

    #[test]
    fn part_one_test() {
        let lines = utils::load_input("inputs/day_11.example").expect("could not load input");
        let parsed = parse_input(&lines).expect("could not parse input");
        assert_eq!(part_one(&parsed.clone()), 10605);
    }

    #[test]
    fn part_two_test() {
        let lines = utils::load_input("inputs/day_11.example").expect("could not load input");
        let parsed = parse_input(&lines).expect("could not parse input");
        assert_eq!(part_two(&parsed), 2713310158);
    }
}
