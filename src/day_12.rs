use std::collections::{BinaryHeap, VecDeque};

use anyhow::{bail, Error, Result};
use rustc_hash::FxHashSet;

use crate::grid::{Coordinate, Grid};

#[derive(Debug, Clone)]
pub struct Terrain {
    grid: Grid<char>,
    start: Coordinate,
    end: Coordinate,
}

impl TryFrom<&[String]> for Terrain {
    type Error = Error;

    fn try_from(lines: &[String]) -> Result<Self, Self::Error> {
        let mut grid: Grid<char> = lines.try_into()?;
        let mut start = None;
        let mut end = None;

        'outer: for i in 0..grid.n {
            for j in 0..grid.m {
                let coord = (i, j).into();
                if grid[coord] == 'S' {
                    start = Some(coord);
                }
                if grid[coord] == 'E' {
                    end = Some(coord);
                }
                if start.is_some() && end.is_some() {
                    break 'outer;
                }
            }
        }

        match (start, end) {
            (Some(s), Some(e)) => {
                grid[s] = 'a';
                grid[e] = 'z';
            }
            _ => bail!("Could not find start or end"),
        }

        Ok(Terrain {
            grid,
            // safe to unwrap because we would have bailed if these were None
            start: start.unwrap(),
            end: end.unwrap(),
        })
    }
}

impl Terrain {
    fn bfs(&self) -> usize {
        let mut visited = FxHashSet::default();
        let mut q = VecDeque::default();
        q.push_back((self.start, 0));

        while let Some((coord, dist)) = q.pop_front() {
            if coord == self.end {
                return dist;
            }

            if visited.contains(&coord) {
                continue;
            }

            visited.insert(coord);

            for neighbour in coord.cardinal_neighbours() {
                if !self.grid.is_in_bounds(neighbour) {
                    continue;
                }

                if self.grid[neighbour] as isize - self.grid[coord] as isize > 1 {
                    continue;
                }

                q.push_back((neighbour, dist + 1));
            }
        }

        usize::MAX
    }

    fn dijkstra(&self) -> usize {
        let mut visited = FxHashSet::default();
        let mut acc = Grid::new(self.grid.n, self.grid.m, usize::MAX);
        let mut q: BinaryHeap<Node> = BinaryHeap::default();
        q.push((0, self.start).into());
        acc[self.start] = 0;

        while let Some(node) = q.pop() {
            let coord = node.coord;
            if coord == self.end {
                return acc[coord];
            }

            if visited.contains(&coord) {
                continue;
            }

            visited.insert(coord);

            for neighbour in coord.cardinal_neighbours() {
                if !self.grid.is_in_bounds(neighbour) {
                    continue;
                }

                if self.grid[neighbour] as isize - self.grid[coord] as isize > 1 {
                    continue;
                }

                if self.grid[neighbour] == 'a' {
                    q.push((0, neighbour).into());
                    acc[neighbour] = 0;
                } else {
                    if acc[coord] + 1 < acc[neighbour] {
                        acc[neighbour] = acc[coord] + 1
                    }

                    q.push((acc[neighbour], neighbour).into())
                }
            }
        }

        acc[self.end]
    }
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq)]
struct Node {
    dist: usize,
    coord: Coordinate,
}

impl From<(usize, Coordinate)> for Node {
    fn from(value: (usize, Coordinate)) -> Self {
        Self {
            dist: value.0,
            coord: value.1,
        }
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.dist.partial_cmp(&self.dist)
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.dist.cmp(&self.dist)
    }
}

pub fn parse_input(lines: &[String]) -> Result<Terrain> {
    lines.try_into()
}

pub fn part_one(parsed: &Terrain) -> usize {
    parsed.bfs()
}

pub fn part_two(parsed: &Terrain) -> usize {
    parsed.dijkstra()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils;

    #[test]
    fn part_one_test() {
        let lines = utils::load_input("inputs/day_12.example").expect("could not load input");
        let parsed = parse_input(&lines).expect("could not parse input");
        assert_eq!(part_one(&parsed), 31);
    }

    #[test]
    fn part_two_test() {
        let lines = utils::load_input("inputs/day_12.example").expect("could not load input");
        let parsed = parse_input(&lines).expect("could not parse input");
        assert_eq!(part_two(&parsed), 29);
    }
}
