use anyhow::{anyhow, Error, Result};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn beats(&self, other: Shape) -> bool {
        (*self == Self::Rock && other == Self::Scissors)
            || (*self == Self::Paper && other == Self::Rock)
            || (*self == Self::Scissors && other == Self::Paper)
    }

    fn score(&self) -> usize {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }
}

impl FromStr for Shape {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "A" | "X" => Ok(Self::Rock),
            "B" | "Y" => Ok(Self::Paper),
            "C" | "Z" => Ok(Self::Scissors),
            _ => Err(anyhow!("Invalid shape code: {}", s)),
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Outcome {
    Lose,
    Draw,
    Win,
}

impl Outcome {
    /// Returns the [Shape] required to achieve this [Outcome] against [opponent_shape]
    fn required_shape(&self, opponent_shape: Shape) -> Shape {
        match (*self, opponent_shape) {
            (Self::Draw, _) => opponent_shape,
            (Self::Lose, Shape::Rock) => Shape::Scissors,
            (Self::Lose, Shape::Paper) => Shape::Rock,
            (Self::Lose, Shape::Scissors) => Shape::Paper,
            (Self::Win, Shape::Rock) => Shape::Paper,
            (Self::Win, Shape::Paper) => Shape::Scissors,
            (Self::Win, Shape::Scissors) => Shape::Rock,
        }
    }
}

impl FromStr for Outcome {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "X" => Ok(Self::Lose),
            "Y" => Ok(Self::Draw),
            "Z" => Ok(Self::Win),
            _ => Err(anyhow!("Invalid outcome code: {}", s)),
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Round {
    opponent_shape: Shape,
    my_shape: Shape,
}

impl Round {
    fn score(&self) -> usize {
        if self.my_shape.beats(self.opponent_shape) {
            self.my_shape.score() + 6
        } else if self.my_shape == self.opponent_shape {
            self.my_shape.score() + 3
        } else {
            self.my_shape.score()
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Rounds {
    shape_round: Round,
    outcome_round: Round,
}

impl FromStr for Rounds {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        s.split_once(" ")
            .and_then(|(a, b)| {
                match (Shape::from_str(a), Shape::from_str(b), Outcome::from_str(b)) {
                    (Ok(opponent_shape), Ok(my_shape), Ok(desired_outcome)) => Some(Rounds {
                        shape_round: Round {
                            opponent_shape,
                            my_shape,
                        },
                        outcome_round: Round {
                            opponent_shape,
                            my_shape: desired_outcome.required_shape(opponent_shape),
                        },
                    }),
                    _ => None,
                }
            })
            .ok_or_else(|| anyhow!("Invalid input for Round: {}", s))
    }
}

pub fn parse_input(lines: Vec<String>) -> Result<Vec<Rounds>> {
    lines.iter().map(|l| Rounds::from_str(l)).collect()
}

pub fn part_one(parsed: &Vec<Rounds>) -> usize {
    parsed.iter().map(|rounds| rounds.shape_round.score()).sum()
}

pub fn part_two(parsed: &Vec<Rounds>) -> usize {
    parsed
        .iter()
        .map(|rounds| rounds.outcome_round.score())
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::utils;

    use super::*;

    #[test]
    fn part_one_test() {
        let lines = utils::load_input("inputs/day_02.example").expect("could not load input");
        let parsed = parse_input(lines).expect("could not parse input");
        assert_eq!(part_one(&parsed), 15);
    }

    #[test]
    fn part_two_test() {
        let lines = utils::load_input("inputs/day_02.example").expect("could not load input");
        let parsed = parse_input(lines).expect("could not parse input");
        assert_eq!(part_two(&parsed), 12);
    }
}
