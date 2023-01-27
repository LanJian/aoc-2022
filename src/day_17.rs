use anyhow::{bail, Error, Result};
use rustc_hash::FxHashMap;
use std::str::FromStr;
use std::{collections::VecDeque, fmt};

use crate::grid::Coordinate;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Direction {
    Left,
    Right,
}

impl TryFrom<char> for Direction {
    type Error = Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '<' => Ok(Self::Left),
            '>' => Ok(Self::Right),
            _ => bail!("Invalid direction"),
        }
    }
}

#[derive(Debug, Clone, Default, Eq, PartialEq)]
struct Wind {
    pattern: Vec<Direction>,
    index: usize,
}

impl FromStr for Wind {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            pattern: s
                .chars()
                .map(|c| Direction::try_from(c))
                .collect::<Result<Vec<_>>>()?,
            index: 0,
        })
    }
}

impl Iterator for Wind {
    type Item = Direction;

    fn next(&mut self) -> Option<Self::Item> {
        self.index %= self.pattern.len();
        let ret = self.pattern[self.index];
        self.index += 1;
        Some(ret)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Shape {
    Bar,
    Plus,
    J,
    I,
    O,
}

impl Shape {
    fn width(&self) -> usize {
        match self {
            Self::Bar => 4,
            Self::Plus => 3,
            Self::J => 3,
            Self::I => 1,
            Self::O => 2,
        }
    }

    fn height(&self) -> usize {
        match self {
            Self::Bar => 1,
            Self::Plus => 3,
            Self::J => 3,
            Self::I => 4,
            Self::O => 2,
        }
    }

    fn rows(&self) -> [u8; 4] {
        match self {
            Self::Bar => [0b11110000, 0b00000000, 0b00000000, 0b00000000],
            Self::Plus => [0b01000000, 0b11100000, 0b01000000, 0b00000000],
            Self::J => [0b00100000, 0b00100000, 0b11100000, 0b00000000],
            Self::I => [0b10000000, 0b10000000, 0b10000000, 0b10000000],
            Self::O => [0b11000000, 0b11000000, 0b00000000, 0b00000000],
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Piece {
    shape: Shape,
    position: Coordinate,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq)]
struct Sky {
    index: usize,
}

impl Sky {
    const PATTERN: [Shape; 5] = [Shape::Bar, Shape::Plus, Shape::J, Shape::I, Shape::O];
}

impl Iterator for Sky {
    type Item = Shape;

    fn next(&mut self) -> Option<Self::Item> {
        self.index %= Self::PATTERN.len();
        let ret = Self::PATTERN[self.index];
        self.index += 1;
        Some(ret)
    }
}

#[derive(Debug, Clone)]
pub struct Tetris {
    sky: Sky,
    wind: Wind,
    well: VecDeque<u8>,
}

impl Tetris {
    const WELL_WIDTH: usize = 7;

    /// Creates space at the top of the well and spawns a new piece at the top
    fn spawn(&mut self) -> Piece {
        let shape = self.sky.next().expect("could not get next shape");

        for _ in 0..shape.height() + 3 {
            self.well.push_front(0b00000000);
        }

        Piece {
            shape,
            position: Coordinate(0, 2),
        }
    }

    /// Simulates pieces dropping for the specified number of rounds and returns the final height
    fn simulate(&mut self, rounds: usize) -> usize {
        let mut i = 0;
        let mut cycle_found = false;
        let mut offset = 0;
        let mut memo = FxHashMap::default();

        while i < rounds {
            let mut piece = self.spawn();
            while self.tick(&mut piece) {}
            self.settle(&piece);

            // We use the Bar shape to check for cycle because it has the highest potential to
            // create a cycle by blocking the next Plus shape.
            if !cycle_found && piece.shape == Shape::Bar {
                let row = self.well[0];
                let zeros = row.count_zeros() - 1;
                let trailing_zeros = row.trailing_zeros();

                // We check if the top row blocks the next Plus shape. If it does, then it
                // is a potential cycle. Otherwise we cannot guarantee it is a cycle.
                //
                // If the top row has less than 3 empty spaces, or if it has 4 occupied spaces in
                // the middle, then it blocks the Plus shape:
                //
                // |...#...|   |...#...|   |...#...|
                // |..###..|   |..###..|   |..###..|
                // |...#...|   |...#...|   |...#...|
                // |.......|   |.......|   |.......|
                // |.#####.|   |.####..|   |..####.|
                //
                // If we have already seen this top row shape that blocks the Plus shape from going
                // down further, and if the wind index is also the same, then we can guarantee that
                // the Plus shape will land in the same position as last time. This also means the
                // subsequent shapes will also land in the same positions. We found our cycle.
                if trailing_zeros == 1 || trailing_zeros == 2 || zeros < 3 {
                    if let Some((prev_depth, prev_round)) = memo.get(&(row, self.wind.index)) {
                        let depth_diff = self.well.len() - prev_depth;
                        let remaining = rounds - i - 1;
                        let cycle = i - prev_round;

                        // Once we find the cycle, we can jump ahead and just simulate the last few
                        // pieces, then add the calculated offset at the end.
                        offset += remaining / cycle * depth_diff;
                        i = rounds - remaining % cycle - 1;
                        cycle_found = true;
                    }

                    memo.insert((row, self.wind.index), (self.well.len(), i));
                }
            }

            i += 1;
        }

        self.well.len() + offset
    }

    /// Apply the next jet of hot gas and try to move down 1 unit
    ///
    /// Each of the 2 movements only occurs if the piece does not go out of bounds nor collides
    /// with another piece. Returns whether we moved down or not.
    fn tick(&mut self, piece: &mut Piece) -> bool {
        let dir = self.wind.next().expect("cound not get next wind direction");

        // first apply the wind
        match dir {
            Direction::Left => {
                // try to move left and check for out of bounds or collision
                piece.position.1 -= 1;
                if piece.position.1 < 0 || self.collides_with(piece) {
                    // move back if we are out of bounds or collided
                    piece.position.1 += 1;
                }
            }
            Direction::Right => {
                // try to move right and check for out of bounds or collision
                piece.position.1 += 1;
                if (piece.position.1 as usize + piece.shape.width()) > Self::WELL_WIDTH
                    || self.collides_with(piece)
                {
                    // move back if we are out of bounds or collided
                    piece.position.1 -= 1;
                }
            }
        }

        // if we are on the floor, then we cannot move down
        if piece.position.row() as usize + piece.shape.height() >= self.well.len() {
            return false;
        }

        // try to move down and check for collision
        piece.position.0 += 1;
        if self.collides_with(piece) {
            // move back up if we collide
            piece.position.0 -= 1;
            false
        } else {
            true
        }
    }

    /// Settle the piece by marking the positions in the well as occupied and culling any empty
    /// rows from the top of the well.
    fn settle(&mut self, piece: &Piece) {
        let piece_rows = piece.shape.rows();

        for i in 0..piece.shape.height() {
            let row = piece.position.row() as usize + i;
            let piece_mask = piece_rows[i] >> piece.position.col();
            self.well[row] |= piece_mask;
        }

        let len = self
            .well
            .iter()
            .position(|x| *x != 0)
            .unwrap_or(self.well.len());

        for _ in 0..len {
            self.well.pop_front();
        }
    }

    /// Returns true if the piece collides with pieces already in the well
    fn collides_with(&self, piece: &Piece) -> bool {
        let piece_rows = piece.shape.rows();

        for i in 0..piece.shape.height() {
            let row = piece.position.row() as usize + i;
            let well_mask = self.well[row];
            let piece_mask = piece_rows[i] >> piece.position.col();

            if well_mask & piece_mask > 0 {
                return true;
            }
        }

        false
    }
}

impl fmt::Display for Tetris {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..self.well.len() {
            let row = &self.well[i];
            write!(f, "{:>13} |{:#010b}|\n", i, row)?
        }

        Ok(())
    }
}

pub fn parse_input(lines: &[String]) -> Result<Tetris> {
    if lines.len() != 1 {
        bail!("Input should have exactly 1 line");
    }

    Ok(Tetris {
        sky: Sky::default(),
        wind: Wind::from_str(&lines[0])?,
        well: VecDeque::default(),
    })
}

pub fn part_one(parsed: &Tetris) -> usize {
    let mut tetris = parsed.clone();
    tetris.simulate(2022)
}

pub fn part_two(parsed: &Tetris) -> usize {
    let mut tetris = parsed.clone();
    tetris.simulate(1000000000000)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils;

    #[test]
    fn part_one_test() {
        let lines = utils::load_input("inputs/day_17.example").expect("could not load input");
        let parsed = parse_input(&lines).expect("could not parse input");
        assert_eq!(part_one(&parsed), 3068);
    }

    #[test]
    fn part_two_test() {
        let lines = utils::load_input("inputs/day_17.example").expect("could not load input");
        let parsed = parse_input(&lines).expect("could not parse input");
        assert_eq!(part_two(&parsed), 1514285714288);
    }
}
