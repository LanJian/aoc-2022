use anyhow::{anyhow, Error, Ok, Result};

#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct Sequence {
    values: Vec<(usize, i64)>,
}

impl Sequence {
    fn displace(&mut self, from: usize, to: usize) {
        if from == to {
            return;
        }

        let displaced_value = self.values[from];

        if from < to {
            for i in from..to {
                self.values[i] = self.values[i + 1];
            }
        } else {
            for i in (to..from).rev() {
                self.values[i + 1] = self.values[i];
            }
        }

        self.values[to] = displaced_value;
    }

    fn decrypt(&mut self) {
        for (_, x) in self.values.iter_mut() {
            *x = *x * 811589153;
        }
    }

    fn mix(&mut self) -> Result<()> {
        let len = self.values.len() as i64;

        for i in 0..self.values.len() {
            let from = self
                .values
                .iter()
                .position(|(j, _)| i == *j)
                .ok_or_else(|| anyhow!("Could not find value"))?;

            let value = self.values[from].1;

            let reduced = value % (len - 1);
            let mut to = from as i64 + reduced;

            if to >= len {
                to = to - len + 1;
            } else if to < 0 {
                to = len + to - 1;
            }

            self.displace(from, to as usize);
        }

        Ok(())
    }

    fn coordinates(&self) -> Result<i64> {
        let zero_index = self
            .values
            .iter()
            .position(|(_, v)| *v == 0)
            .ok_or_else(|| anyhow!("Value 0 does not exist"))?;

        let len = self.values.len();
        let ret = self.values[(zero_index + 1000) % len].1
            + self.values[(zero_index + 2000) % len].1
            + self.values[(zero_index + 3000) % len].1;

        Ok(ret)
    }
}

impl TryFrom<&[String]> for Sequence {
    type Error = Error;

    fn try_from(lines: &[String]) -> Result<Self, Self::Error> {
        let tmp: Vec<i64> = lines
            .iter()
            .map(|l| l.parse())
            .collect::<Result<Vec<_>, _>>()?;

        let values = tmp.iter().enumerate().map(|(i, v)| (i, *v)).collect();

        Ok(Self { values })
    }
}

pub fn parse_input(lines: &[String]) -> Result<Sequence> {
    Sequence::try_from(lines)
}

pub fn part_one(parsed: &Sequence) -> i64 {
    let mut sequence = parsed.clone();
    sequence.mix().expect("Could not perform mix");
    sequence.coordinates().expect("Could not find coordinates")
}

pub fn part_two(parsed: &Sequence) -> i64 {
    let mut sequence = parsed.clone();
    sequence.decrypt();

    for _ in 0..10 {
        sequence.mix().expect("Could not perform mix");
    }

    sequence.coordinates().expect("Could not find coordinates")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils;

    #[test]
    fn part_one_test() {
        let lines = utils::load_input("inputs/day_20.example").expect("could not load input");
        let parsed = parse_input(&lines).expect("could not parse input");
        assert_eq!(part_one(&parsed), 3);
    }

    #[test]
    fn part_two_test() {
        let lines = utils::load_input("inputs/day_20.example").expect("could not load input");
        let parsed = parse_input(&lines).expect("could not parse input");
        assert_eq!(part_two(&parsed), 1623178306);
    }
}
