use anyhow::{bail, Error, Result};
use std::fmt;
use std::iter::Sum;
use std::ops::Add;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq)]
struct Snafu(i64);

impl FromStr for Snafu {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut number = 0;

        for c in s.chars() {
            number *= 5;

            let digit = match c {
                '=' => -2,
                '-' => -1,
                '0' => 0,
                '1' => 1,
                '2' => 2,
                _ => bail!("Invalid SNAFU number"),
            };

            number += digit;
        }

        Ok(Self(number))
    }
}

impl Add for Snafu {
    type Output = Snafu;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl<'a> Sum<&'a Snafu> for Snafu {
    fn sum<I: Iterator<Item = &'a Snafu>>(iter: I) -> Self {
        iter.fold(Snafu::default(), |acc, x| acc + *x)
    }
}

impl fmt::Display for Snafu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut chars: Vec<char> = Vec::default();
        let mut acc = self.0;

        while acc > 0 {
            let rem = acc % 5;

            let (c, carry) = match rem {
                0 => ('0', 0),
                1 => ('1', 0),
                2 => ('2', 0),
                3 => ('=', 2),
                4 => ('-', 1),
                _ => unreachable!(),
            };

            chars.push(c);
            acc = (acc + carry) / 5;
        }

        write!(f, "{}", chars.iter().rev().collect::<String>())
    }
}

#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct Bob {
    fuel_requirements: Vec<Snafu>,
}

impl Bob {
    fn fuel_sum(&self) -> Snafu {
        self.fuel_requirements.iter().sum()
    }
}

impl TryFrom<&[String]> for Bob {
    type Error = Error;

    fn try_from(lines: &[String]) -> Result<Self, Self::Error> {
        Ok(Bob {
            fuel_requirements: lines
                .iter()
                .map(|l| Snafu::from_str(l))
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

pub fn parse_input(lines: &[String]) -> Result<Bob> {
    lines.try_into()
}

pub fn part_one(parsed: &Bob) -> String {
    parsed.fuel_sum().to_string()
}

pub fn part_two(parsed: &Bob) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils;

    #[test]
    fn snafu_test() {
        assert_eq!(Snafu::from_str("1").unwrap(), Snafu(1));
        assert_eq!(Snafu::from_str("2").unwrap(), Snafu(2));
        assert_eq!(Snafu::from_str("1=").unwrap(), Snafu(3));
        assert_eq!(Snafu::from_str("1-").unwrap(), Snafu(4));
        assert_eq!(Snafu::from_str("10").unwrap(), Snafu(5));
        assert_eq!(Snafu::from_str("11").unwrap(), Snafu(6));
        assert_eq!(Snafu::from_str("12").unwrap(), Snafu(7));
        assert_eq!(Snafu::from_str("2=").unwrap(), Snafu(8));
        assert_eq!(Snafu::from_str("2-").unwrap(), Snafu(9));
        assert_eq!(Snafu::from_str("20").unwrap(), Snafu(10));
        assert_eq!(Snafu::from_str("1=0").unwrap(), Snafu(15));
        assert_eq!(Snafu::from_str("1-0").unwrap(), Snafu(20));
        assert_eq!(Snafu::from_str("1=11-2").unwrap(), Snafu(2022));
        assert_eq!(Snafu::from_str("1-0---0").unwrap(), Snafu(12345));
        assert_eq!(Snafu::from_str("1121-1110-1=0").unwrap(), Snafu(314159265));

        assert_eq!(Snafu(1).to_string(), "1");
        assert_eq!(Snafu(2).to_string(), "2");
        assert_eq!(Snafu(3).to_string(), "1=");
        assert_eq!(Snafu(4).to_string(), "1-");
        assert_eq!(Snafu(5).to_string(), "10");
        assert_eq!(Snafu(6).to_string(), "11");
        assert_eq!(Snafu(7).to_string(), "12");
        assert_eq!(Snafu(8).to_string(), "2=");
        assert_eq!(Snafu(9).to_string(), "2-");
        assert_eq!(Snafu(10).to_string(), "20");
        assert_eq!(Snafu(15).to_string(), "1=0");
        assert_eq!(Snafu(20).to_string(), "1-0");
        assert_eq!(Snafu(2022).to_string(), "1=11-2");
        assert_eq!(Snafu(12345).to_string(), "1-0---0");
        assert_eq!(Snafu(314159265).to_string(), "1121-1110-1=0");
    }

    #[test]
    fn part_one_test() {
        let lines = utils::load_input("inputs/day_25.example").expect("could not load input");
        let parsed = parse_input(&lines).expect("could not parse input");
        assert_eq!(part_one(&parsed), "2=-1=0");
    }

    #[test]
    fn part_two_test() {
        let lines = utils::load_input("inputs/day_25.example").expect("could not load input");
        let parsed = parse_input(&lines).expect("could not parse input");
        assert_eq!(part_two(&parsed), 0);
    }
}
