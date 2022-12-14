use anyhow::{anyhow, Error, Result};

use crate::grid::{Coordinate, Grid};

#[derive(Debug, Clone)]
pub struct Cave {
    grid: Grid<bool>,
    sand_origin: Coordinate,
    num_sand_grains: usize,
}

impl TryFrom<&[String]> for Cave {
    type Error = Error;

    fn try_from(lines: &[String]) -> Result<Self, Self::Error> {
        let mut max_row = usize::MIN;
        let mut min_col = usize::MAX;
        let mut max_col = usize::MIN;
        let mut segments: Vec<(Coordinate, Coordinate)> = Vec::default();

        for l in lines {
            let mut prev_row = 0_usize;
            let mut prev_col = 0_usize;

            for (i, token) in l.split(" -> ").enumerate() {
                let (a, b) = token
                    .split_once(',')
                    .ok_or_else(|| anyhow!("Could not parse coordinate: {}", token))?;
                let col: usize = a.parse()?;
                let row: usize = b.parse()?;

                if i > 0 {
                    segments.push(((prev_row, prev_col).into(), (row, col).into()));
                }

                prev_row = row;
                prev_col = col;

                if row > max_row {
                    max_row = row;
                }

                if col < min_col {
                    min_col = col;
                }

                if col > max_col {
                    max_col = col;
                }
            }
        }

        // extend the sides by 1 column each side so we can correctly simulate for part 2
        min_col -= 1;
        max_col += 1;
        let n = max_row + 3;
        let mut grid = Grid::new(n, max_col - min_col + 1, false);

        // now fill in the rocks
        for (a, b) in segments {
            if a.row() == b.row() {
                let min = a.col().min(b.col());
                let max = a.col().max(b.col());
                for j in min..=max {
                    let coord = (a.row(), j - min_col as isize).into();
                    grid[coord] = true;
                }
            } else if a.col() == b.col() {
                let min = a.row().min(b.row());
                let max = a.row().max(b.row());
                for i in min..=max {
                    let coord = (i, a.col() - min_col as isize).into();
                    grid[coord] = true;
                }
            }
        }

        Ok(Self {
            grid,
            sand_origin: (0, 500 - min_col).into(),
            num_sand_grains: 0,
        })
    }
}

impl Cave {
    fn fill_sand(&mut self, bottomless: bool) {
        loop {
            match self.drop_sand(bottomless) {
                Some(coord) => {
                    self.grid[coord] = true;
                    self.num_sand_grains += 1;

                    // if the origin is blocked, then we cannot drop any more sand
                    if coord == self.sand_origin {
                        break;
                    }
                }
                None => break,
            }
        }

        // if there is a floor, add the sides
        if !bottomless {
            let h = self.grid.n - 2;
            let w = self.grid.m;
            let o = self.sand_origin.col() as usize;
            let left = h - o;
            let right = h - (w - o) + 1;

            self.num_sand_grains += (1 + left) * left / 2;
            self.num_sand_grains += (1 + right) * right / 2;
        }
    }

    fn drop_sand(&mut self, bottomless: bool) -> Option<Coordinate> {
        let mut cur = self.sand_origin.clone();
        loop {
            let new_coord = self.tick(cur, bottomless);
            if new_coord.is_none() {
                return None;
            }

            // safe to unwrap because we return if it was None
            let unwrapped = new_coord.unwrap();
            if unwrapped == cur {
                break;
            }

            cur = unwrapped;
        }

        Some(cur)
    }

    fn tick(&mut self, sand: Coordinate, bottomless: bool) -> Option<Coordinate> {
        let candidates = [sand.south(), sand.southwest(), sand.southeast()];
        for c in candidates {
            if bottomless {
                // if we go out of bounds, then it will fall forever
                if !self.grid.is_in_bounds(c) {
                    return None;
                }

                // if the candidate is unoccupied, we will go there
                if self.grid[c] == false {
                    return Some(c);
                }
            } else {
                // if we go out of bounds, we assume it's occupied
                if !self.grid.is_in_bounds(c) {
                    continue;
                }

                // if the candidate is on the floor, then we cannot go down anymore
                let floor = self.grid.n - 1;
                if c.row() as usize == floor {
                    return Some(sand);
                }

                // if the candidate is unoccupied, we will go there
                if self.grid[c] == false {
                    return Some(c);
                }
            }
        }

        // if we didn't return yet, then we couldn't go further, so we rest at the same location
        Some(sand)
    }
}

pub fn parse_input(lines: &[String]) -> Result<Cave> {
    Cave::try_from(lines)
}

pub fn part_one(parsed: &Cave) -> usize {
    let mut cave = parsed.clone();
    cave.fill_sand(true);
    cave.num_sand_grains
}

pub fn part_two(parsed: &Cave) -> usize {
    let mut cave = parsed.clone();
    cave.fill_sand(false);
    cave.num_sand_grains
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils;

    #[test]
    fn part_one_test() {
        let lines = utils::load_input("inputs/day_14.example").expect("could not load input");
        let parsed = parse_input(&lines).expect("could not parse input");
        assert_eq!(part_one(&parsed), 24);
    }

    #[test]
    fn part_two_test() {
        let lines = utils::load_input("inputs/day_14.example").expect("could not load input");
        let parsed = parse_input(&lines).expect("could not parse input");
        assert_eq!(part_two(&parsed), 93);
    }
}
