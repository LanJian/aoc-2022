use anyhow::{Error, Result};
use rustc_hash::{FxHashMap, FxHashSet};

use crate::grid::Coordinate;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Direction {
    North,
    South,
    West,
    East,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct StartingDirection {
    directions: [Direction; 4],
    index: usize,
}

impl StartingDirection {
    pub fn new() -> Self {
        Self {
            directions: [
                Direction::North,
                Direction::South,
                Direction::West,
                Direction::East,
            ],
            index: 0,
        }
    }
}

impl Iterator for StartingDirection {
    type Item = Direction;

    fn next(&mut self) -> Option<Self::Item> {
        self.index %= self.directions.len();
        let ret = self.directions[self.index];
        self.index += 1;
        Some(ret)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Proposal {
    position: Coordinate,
    starting_dir: Direction,
    directions: [Direction; 4],
    index: usize,
}

impl Proposal {
    pub fn new(position: Coordinate, starting_dir: Direction) -> Self {
        let directions = match starting_dir {
            Direction::North => [
                Direction::North,
                Direction::South,
                Direction::West,
                Direction::East,
            ],
            Direction::South => [
                Direction::South,
                Direction::West,
                Direction::East,
                Direction::North,
            ],
            Direction::West => [
                Direction::West,
                Direction::East,
                Direction::North,
                Direction::South,
            ],
            Direction::East => [
                Direction::East,
                Direction::North,
                Direction::South,
                Direction::West,
            ],
        };
        Self {
            position,
            starting_dir,
            directions,
            index: 0,
        }
    }
}

impl Iterator for Proposal {
    type Item = [Coordinate; 3];

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.directions.len() {
            return None;
        }

        let ret = match self.directions[self.index] {
            Direction::North => [
                self.position.north(),
                self.position.northeast(),
                self.position.northwest(),
            ],
            Direction::South => [
                self.position.south(),
                self.position.southeast(),
                self.position.southwest(),
            ],
            Direction::West => [
                self.position.west(),
                self.position.northwest(),
                self.position.southwest(),
            ],
            Direction::East => [
                self.position.east(),
                self.position.northeast(),
                self.position.southeast(),
            ],
        };

        self.index += 1;
        Some(ret)
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Grove {
    elves: FxHashSet<Coordinate>,
    starting_dir: StartingDirection,
    proposed: FxHashMap<Coordinate, Coordinate>,
    seen: FxHashSet<Coordinate>
}

impl Grove {
    fn propose(&mut self) {
        let starting_dir = self
            .starting_dir
            .next()
            .expect("could not get starting dir");

        for coord in &self.elves {
            if coord.neighbours().iter().any(|n| self.elves.contains(n)) {
                for coords in Proposal::new(*coord, starting_dir) {
                    if coords.iter().any(|c| self.elves.contains(c)) {
                        continue;
                    }

                    if self.seen.contains(&coords[0]) {
                        self.proposed.remove(&coords[0]);
                        break;
                    }

                    self.seen.insert(coords[0]);
                    self.proposed.insert(coords[0], *coord);
                    break;
                }
            }
        }
    }

    fn execute(&mut self) {
        for (to, from) in self.proposed.drain() {
            self.elves.remove(&from);
            self.elves.insert(to);
        }
        self.seen.clear();
    }

    fn disperse(&mut self, rounds: usize) {
        for _ in 0..rounds {
            self.propose();
            self.execute();
        }
    }

    fn fully_disperse(&mut self) -> usize {
        let mut rounds = 1;

        loop {
            self.propose();

            if self.proposed.is_empty() {
                break;
            }

            self.execute();
            rounds += 1;
        }

        rounds
    }

    fn count_empty(&self) -> usize {
        let mut min_row = isize::MAX;
        let mut min_col = isize::MAX;
        let mut max_row = isize::MIN;
        let mut max_col = isize::MIN;

        for coord in &self.elves {
            min_row = min_row.min(coord.row());
            min_col = min_col.min(coord.col());
            max_row = max_row.max(coord.row());
            max_col = max_col.max(coord.col());
        }

        (max_row - min_row + 1) as usize * (max_col - min_col + 1) as usize - self.elves.len()
    }
}

impl TryFrom<&[String]> for Grove {
    type Error = Error;

    fn try_from(lines: &[String]) -> Result<Self, Self::Error> {
        let mut elves = FxHashSet::default();

        for (i, l) in lines.iter().enumerate() {
            for (j, c) in l.char_indices() {
                if c == '#' {
                    elves.insert((i, j).into());
                }
            }
        }

        Ok(Self {
            elves,
            starting_dir: StartingDirection::new(),
            proposed: FxHashMap::default(),
            seen: FxHashSet::default()
        })
    }
}

pub fn parse_input(lines: &[String]) -> Result<Grove> {
    Grove::try_from(lines)
}

pub fn part_one(parsed: &Grove) -> usize {
    let mut grove = parsed.clone();
    grove.disperse(10);
    grove.count_empty()
}

pub fn part_two(parsed: &Grove) -> usize {
    let mut grove = parsed.clone();
    grove.fully_disperse()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils;

    #[test]
    fn part_one_test() {
        let lines = utils::load_input("inputs/day_23.example").expect("could not load input");
        let parsed = parse_input(&lines).expect("could not parse input");
        assert_eq!(part_one(&parsed), 110);
    }

    #[test]
    fn part_two_test() {
        let lines = utils::load_input("inputs/day_23.example").expect("could not load input");
        let parsed = parse_input(&lines).expect("could not parse input");
        assert_eq!(part_two(&parsed), 20);
    }
}
