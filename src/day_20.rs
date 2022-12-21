use anyhow::{anyhow, Error, Result};
use rustc_hash::FxHashMap;

#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct Sequence {
    values: Vec<i64>,
    original: Vec<i64>,
    indices: FxHashMap<i64, FxHashMap<usize, usize>>,
}

impl Sequence {
    fn build_indices(&mut self) {
        for (i, value) in self.values.iter().enumerate() {
            self.indices
                .entry(*value)
                .and_modify(|e| {
                    e.insert(i, i);
                })
                .or_insert(FxHashMap::from_iter([(i, i)]));
        }
    }

    fn swap(&mut self, i: usize, j: usize) {
        if i == j {
            return;
        }

        let a = self.values[i];
        let b = self.values[j];

        self.values[i] = b;
        self.values[j] = a;
    }

    fn update_index(&mut self, value: i64, cur_index: usize, new_index: usize) {
        self.indices.entry(value).and_modify(|e| {
            if let Some((_, foo)) = e.iter_mut().find(|(_, v)| **v == cur_index) {
                *foo = new_index;
            }
        });
    }

    fn displace(&mut self, from: usize, to: usize) {
        if from == to {
            return;
        }

        // temporarily update the index to something out of bounds to avoid collision
        let displaced_value = self.values[from];
        self.update_index(displaced_value, from, self.values.len());

        if from < to {
            for i in from..to {
                let j = i + 1;
                let value = self.values[j];
                self.swap(i, j);
                self.update_index(value, j, i);
            }
        } else {
            for i in (to..from).rev() {
                let j = i + 1;
                let value = self.values[i];
                self.swap(i, j);
                self.update_index(value, i, j);
            }
        }

        self.update_index(displaced_value, self.values.len(), to);
    }

    fn decrypt(&mut self) {
        for x in self.values.iter_mut() {
            *x = *x * 811589153;
        }

        for x in self.original.iter_mut() {
            *x = *x * 811589153;
        }
    }

    fn mix(&mut self) {
        let len = self.original.len() as i64;

        for i in 0..self.original.len() {
            let value = self.original[i];
            let from = self.indices[&value][&i];
            let reduced = value % (len - 1);
            let mut to = from as i64 + reduced;

            if to >= len {
                to = to - len + 1;
            } else if to < 0 {
                to = len + to - 1;
            }

            self.displace(from, to as usize);
        }
    }

    fn coordinates(&self) -> Result<i64> {
        let zero_index = self.indices[&0]
            .values()
            .next()
            .ok_or_else(|| anyhow!("Value 0 does not exist"))?;

        let len = self.values.len();
        let ret = self.values[(*zero_index + 1000) % len]
            + self.values[(*zero_index + 2000) % len]
            + self.values[(*zero_index + 3000) % len];

        Ok(ret)
    }
}

impl TryFrom<&[String]> for Sequence {
    type Error = Error;

    fn try_from(lines: &[String]) -> Result<Self, Self::Error> {
        let values: Vec<i64> = lines
            .iter()
            .map(|l| l.parse())
            .collect::<Result<Vec<_>, _>>()?;

        let original = values.clone();
        let indices: FxHashMap<i64, FxHashMap<usize, usize>> = FxHashMap::default();

        Ok(Self { values, original, indices })
    }
}

pub fn parse_input(lines: &[String]) -> Result<Sequence> {
    Sequence::try_from(lines)
}

pub fn part_one(parsed: &Sequence) -> i64 {
    let mut sequence = parsed.clone();
    sequence.build_indices();
    sequence.mix();
    sequence.coordinates().expect("Could not find coordinates")
}

pub fn part_two(parsed: &Sequence) -> i64 {
    let mut sequence = parsed.clone();
    sequence.decrypt();
    sequence.build_indices();

    for _ in 0..10 {
        sequence.mix();
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
