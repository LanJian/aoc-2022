use anyhow::{anyhow, Error, Result};
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Default, Eq, PartialEq)]
struct SupplyStack {
    stack: Vec<char>,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq)]
struct Procedure {
    from: usize,
    to: usize,
    quantity: usize,
}

impl FromStr for Procedure {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.split(" ");

        tokens
            .next()
            .ok_or_else(|| anyhow!("Invalid procedure input: {}", s))?;
        let quantity = tokens
            .next()
            .and_then(|t| t.parse::<usize>().ok())
            .ok_or_else(|| anyhow!("Could not parse `quantity` from input: {}", s))?;

        tokens
            .next()
            .ok_or_else(|| anyhow!("Invalid procedure input: {}", s))?;
        let from = tokens
            .next()
            .and_then(|t| t.parse::<usize>().ok())
            .ok_or_else(|| anyhow!("Could not parse `from` from input: {}", s))?;

        tokens
            .next()
            .ok_or_else(|| anyhow!("Invalid procedure input: {}", s))?;
        let to = tokens
            .next()
            .and_then(|t| t.parse::<usize>().ok())
            .ok_or_else(|| anyhow!("Could not parse `to` from input: {}", s))?;

        // we use 0 index
        Ok(Self {
            from: from - 1,
            to: to - 1,
            quantity,
        })
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Mover {
    CrateMover9000,
    CrateMover9001,
}

impl Mover {
    /// Applies the provided [procedure] to the [supply_stacks]
    fn apply_procedure(&self, procedure: &Procedure, supply_stacks: &mut Vec<SupplyStack>) {
        match self {
            Self::CrateMover9000 => {
                for _ in 0..procedure.quantity {
                    let label = supply_stacks
                        .get_mut(procedure.from)
                        .and_then(|s| s.stack.pop());
                    if let Some(l) = label {
                        if let Some(s) = supply_stacks.get_mut(procedure.to) {
                            s.stack.push(l);
                        }
                    }
                }
            }
            Self::CrateMover9001 => {
                for i in 0..procedure.quantity {
                    let label = supply_stacks
                        .get_mut(procedure.from)
                        .and_then(|s| {
                            let len = s.stack.len();
                            s.stack.get(len - procedure.quantity + i)
                        })
                        .map(|l| *l);
                    if let Some(l) = label {
                        if let Some(s) = supply_stacks.get_mut(procedure.to) {
                            s.stack.push(l);
                        }
                    }
                }
                for _ in 0..procedure.quantity {
                    supply_stacks
                        .get_mut(procedure.from)
                        .and_then(|s| s.stack.pop());
                }
            }
        }
    }
}

#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct Ship {
    supply_stacks: Vec<SupplyStack>,

    /// Procedures are in reverse order
    procedures: Vec<Procedure>,
}

impl Ship {
    /// Applies the stored [procedures] to the [supply_stacks] with the provided [mover]
    fn apply_procedures_with(&mut self, mover: Mover) {
        for procedure in self.procedures.iter().rev() {
            mover.apply_procedure(procedure, &mut self.supply_stacks);
        }
    }
}

impl fmt::Display for Ship {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.supply_stacks
                .iter()
                .filter_map(|s| s.stack.last())
                .collect::<String>()
        )
    }
}

pub fn parse_input(lines: &[String]) -> Result<Ship> {
    let mut iter = lines.iter().rev();

    // procedures are in reverse order
    let mut procedures = Vec::default();
    while let Some(l) = iter.next() {
        if l == "" {
            break;
        }

        procedures.push(Procedure::from_str(l)?);
    }

    let l = iter.next().ok_or_else(|| anyhow!("Invalid input"))?;
    let count = l.split_whitespace().count();

    let mut supply_stacks = vec![SupplyStack::default(); count];
    for l in iter {
        for (i, label) in l.chars().skip(1).step_by(4).enumerate() {
            if !label.is_whitespace() {
                supply_stacks[i].stack.push(label);
            }
        }
    }

    Ok(Ship {
        supply_stacks,
        procedures,
    })
}

pub fn part_one(parsed: &mut Ship) -> String {
    parsed.apply_procedures_with(Mover::CrateMover9000);
    parsed.to_string()
}

pub fn part_two(parsed: &mut Ship) -> String {
    parsed.apply_procedures_with(Mover::CrateMover9001);
    parsed.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils;

    #[test]
    fn part_one_test() {
        let lines = utils::load_input("inputs/day_05.example").expect("could not load input");
        let mut parsed = parse_input(&lines).expect("could not parse input");
        assert_eq!(part_one(&mut parsed), "CMZ");
    }

    #[test]
    fn part_two_test() {
        let lines = utils::load_input("inputs/day_05.example").expect("could not load input");
        let mut parsed = parse_input(&lines).expect("could not parse input");
        assert_eq!(part_two(&mut parsed), "MCD");
    }
}
