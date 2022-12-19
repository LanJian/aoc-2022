use std::str::FromStr;

use anyhow::{Error, Result};

use crate::algebra::Point3;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Block {
    Interior,
    Exterior,
    Lava,
}

#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct Lava {
    points: Vec<Point3>,
    grid: Vec<Vec<Vec<Block>>>,
    n: usize,
    m: usize,
    l: usize,
}

impl Lava {
    fn surface_area(&self) -> usize {
        let mut count = 0;
        for p in &self.points {
            count += p
                .cardinal_neighbours()
                .iter()
                .filter(|&q| {
                    let (i, j, k) = (q.x as usize, q.y as usize, q.z as usize);
                    !self.is_in_bounds(q) || self.grid[i][j][k] != Block::Lava
                })
                .count()
        }

        count
    }

    fn exterior_surface_area(&mut self) -> usize {
        self.fill_exterior();

        let mut count = 0;
        for p in &self.points {
            count += p
                .cardinal_neighbours()
                .iter()
                .filter(|&q| {
                    let (i, j, k) = (q.x as usize, q.y as usize, q.z as usize);
                    !self.is_in_bounds(q) || self.grid[i][j][k] == Block::Exterior
                })
                .count()
        }

        count
    }

    fn fill_exterior(&mut self) {
        // do dfs from all 6 sides
        for i in 0..self.n {
            for j in 0..self.m {
                self.dfs(&Point3::new(i as i64, j as i64, 0));
            }
        }

        for i in 0..self.n {
            for j in 0..self.m {
                self.dfs(&Point3::new(i as i64, j as i64, self.l as i64 - 1));
            }
        }

        for i in 0..self.n {
            for k in 0..self.l {
                self.dfs(&Point3::new(i as i64, 0, k as i64));
            }
        }

        for i in 0..self.n {
            for k in 0..self.l {
                self.dfs(&Point3::new(i as i64, self.m as i64 - 1, k as i64));
            }
        }

        for j in 0..self.m {
            for k in 0..self.l {
                self.dfs(&Point3::new(0, j as i64, k as i64));
            }
        }

        for j in 0..self.m {
            for k in 0..self.l {
                self.dfs(&Point3::new(self.n as i64 - 1, j as i64, k as i64));
            }
        }
    }

    fn dfs(&mut self, p: &Point3) {
        if !self.is_in_bounds(p) {
            return;
        }

        let (i, j, k) = (p.x as usize, p.y as usize, p.z as usize);

        if self.grid[i][j][k] == Block::Exterior || self.grid[i][j][k] == Block::Lava {
            return;
        }

        self.grid[i][j][k] = Block::Exterior;

        for neighbour in p.cardinal_neighbours() {
            self.dfs(&neighbour);
        }
    }

    fn is_in_bounds(&self, p: &Point3) -> bool {
        p.x >= 0
            && p.x < self.n as i64
            && p.y >= 0
            && p.y < self.m as i64
            && p.z >= 0
            && p.z < self.l as i64
    }
}

impl TryFrom<&[String]> for Lava {
    type Error = Error;

    fn try_from(lines: &[String]) -> Result<Self, Self::Error> {
        let points = lines
            .iter()
            .map(|l| Point3::from_str(l))
            .collect::<Result<Vec<_>, _>>()?;

        let (mut n, mut m, mut l) = (i64::MIN, i64::MIN, i64::MIN);

        for p in &points {
            n = n.max(p.x);
            m = m.max(p.y);
            l = l.max(p.z);
        }

        n += 1;
        m += 1;
        l += 1;

        // We are assuming that we don't have negative coordinates. If we do, then you will need
        // to do some extra math to calculate the grid size and the indices when accessing the
        // blocks.
        let mut grid = vec![vec![vec![Block::Interior; l as usize]; m as usize]; n as usize];

        for p in &points {
            let (i, j, k) = (p.x as usize, p.y as usize, p.z as usize);
            grid[i][j][k] = Block::Lava;
        }

        Ok(Lava {
            points,
            grid,
            n: n as usize,
            m: m as usize,
            l: l as usize,
        })
    }
}

pub fn parse_input(lines: &[String]) -> Result<Lava> {
    Lava::try_from(lines)
}

pub fn part_one(parsed: &Lava) -> usize {
    parsed.surface_area()
}

pub fn part_two(parsed: &Lava) -> usize {
    let mut lava = parsed.clone();
    lava.exterior_surface_area()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils;

    #[test]
    fn part_one_test() {
        let lines = utils::load_input("inputs/day_18.example").expect("could not load input");
        let parsed = parse_input(&lines).expect("could not parse input");
        assert_eq!(part_one(&parsed), 64);
    }

    #[test]
    fn part_two_test() {
        let lines = utils::load_input("inputs/day_18.example").expect("could not load input");
        let parsed = parse_input(&lines).expect("could not parse input");
        assert_eq!(part_two(&parsed), 58);
    }
}
