use anyhow::{anyhow, bail, Error, Result};
use std::str::FromStr;

use crate::grid::{Coordinate, Grid};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Tile {
    Void,
    Wall,
    Open {
        north: isize,
        south: isize,
        west: isize,
        east: isize,
    },
}

impl Tile {
    fn is_open(&self) -> bool {
        match self {
            Self::Open { .. } => true,
            _ => false,
        }
    }

    fn is_wall(&self) -> bool {
        match self {
            Self::Wall => true,
            _ => false,
        }
    }
    fn is_void(&self) -> bool {
        match self {
            Self::Void => true,
            _ => false,
        }
    }
}

impl Default for Tile {
    fn default() -> Self {
        Tile::Void
    }
}

impl TryFrom<char> for Tile {
    type Error = Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            ' ' => Self::Void,
            '#' => Self::Wall,
            '.' => Self::Open {
                north: isize::MIN,
                south: isize::MAX,
                west: isize::MIN,
                east: isize::MAX,
            },
            _ => bail!("Invalid char"),
        })
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Turn {
    Clockwise,
    Counterclockwise,
}

impl TryFrom<char> for Turn {
    type Error = Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            'R' => Self::Clockwise,
            'L' => Self::Counterclockwise,
            _ => bail!("Invalid char"),
        })
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    fn turn(&self, turn: &Turn) -> Self {
        match (self, turn) {
            (Self::North, Turn::Clockwise) => Self::East,
            (Self::East, Turn::Clockwise) => Self::South,
            (Self::South, Turn::Clockwise) => Self::West,
            (Self::West, Turn::Clockwise) => Self::North,
            (Self::North, Turn::Counterclockwise) => Self::West,
            (Self::East, Turn::Counterclockwise) => Self::North,
            (Self::South, Turn::Counterclockwise) => Self::East,
            (Self::West, Turn::Counterclockwise) => Self::South,
        }
    }

    fn value(&self) -> usize {
        match self {
            Self::East => 0,
            Self::South => 1,
            Self::West => 2,
            Self::North => 3,
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Instruction {
    steps: usize,
    turn: Turn,
}

impl FromStr for Instruction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let res = s
            .chars()
            .rev()
            .next()
            .ok_or_else(|| anyhow!("Invalid instruction"))?
            .try_into();

        Ok(match res {
            // if we could parse the turn, then we are still in the middle of the line
            Ok(turn) => Self {
                steps: s[0..s.len() - 1].parse()?,
                turn,
            },
            // if we couldn't parse the turn, then we are at the end of the line, just use a dummy
            // value
            Err(_) => Self {
                steps: s.parse()?,
                turn: Turn::Clockwise,
            },
        })
    }
}

#[derive(Debug, Clone)]
pub struct Board {
    grid: Grid<Tile>,
    wall_rows: Vec<Vec<usize>>,
    wall_cols: Vec<Vec<usize>>,
    row_bounds: Vec<(usize, usize)>,
    col_bounds: Vec<(usize, usize)>,
    start: Coordinate,
    route: Vec<Instruction>,
}

impl Board {
    fn preprocess(&mut self) {
        let mut wall_col_indices: Vec<usize> = vec![0; self.grid.m];

        for i in 0..self.grid.n {
            let (west_bound, east_bound) = self.row_bounds[i];
            let wall_row = &self.wall_rows[i];
            let mut wall_row_index = 0;

            for j in 0..self.grid.m {
                let tile = &mut self.grid[(i, j).into()];

                let (north_bound, south_bound) = self.col_bounds[j];
                let wall_col = &self.wall_cols[j];
                let wall_col_index = wall_col_indices[j];

                match tile {
                    Tile::Void => continue,
                    Tile::Wall => {
                        wall_row_index = wall_row_index + 1;
                        wall_col_indices[j] = wall_col_index + 1;
                    }
                    Tile::Open {
                        north,
                        south,
                        west,
                        east,
                    } => {
                        if wall_row_index > 0 {
                            *west = wall_row[wall_row_index - 1] as isize + 1;
                        } else {
                            *west = west_bound as isize;
                        }

                        if wall_row_index < wall_row.len() {
                            *east = wall_row[wall_row_index] as isize - 1;
                        } else {
                            *east = east_bound as isize;
                        }

                        if wall_col_index > 0 {
                            *north = wall_col[wall_col_index - 1] as isize + 1;
                        } else {
                            *north = north_bound as isize;
                        }

                        if wall_col_index < wall_col.len() {
                            *south = wall_col[wall_col_index] as isize - 1;
                        } else {
                            *south = south_bound as isize;
                        }
                    }
                }
            }
        }
    }


    fn wrap_around_2d(&self, position: &Coordinate, dir: &Direction) -> Option<(Coordinate, Direction)> {
        let row = position.row() as usize;
        let col = position.col() as usize;

        let coord = match dir {
            Direction::East => (row, self.row_bounds[row].0).into(),
            Direction::West => (row, self.row_bounds[row].1).into(),
            Direction::South => (self.col_bounds[col].0, col).into(),
            Direction::North => (self.col_bounds[col].1, col).into(),
        };

        Some((coord, *dir))
    }

    fn wrap_around_3d(
        &self,
        position: &Coordinate,
        dir: &Direction,
    ) -> Option<(Coordinate, Direction)> {
        // XXX jackhxs 2022-12-27: We are making the assumption that the input always follow this
        // layout:
        //
        //     #########
        //     # 1 # 2 #
        //     #########
        //     # 3 #
        // #########
        // # 4 # 5 #
        // #########
        // # 6 #
        // #####

        let row = position.row() as usize;
        let col = position.col() as usize;
        let edge_len = self.grid.m / 3;

        // 1 north edge <-> 6 west edge
        if *dir == Direction::North && row == 0 && (edge_len..edge_len * 2).contains(&col) {
            let i = col - edge_len;
            return Some(((edge_len * 3 + i, 0).into(), Direction::East));
        }

        if *dir == Direction::West && col == 0 && (edge_len * 3..edge_len * 4).contains(&row) {
            let i = row - edge_len * 3;
            return Some(((0, edge_len + i).into(), Direction::South));
        }

        // 2 north edge <-> 6 south edge
        if *dir == Direction::North && row == 0 && (edge_len * 2..edge_len * 3).contains(&col) {
            let i = col - edge_len * 2;
            return Some(((edge_len * 4 - 1, i).into(), Direction::North));
        }

        if *dir == Direction::South && row == edge_len * 4 - 1 && (0..edge_len).contains(&col) {
            let i = col;
            return Some(((0, edge_len * 2 + i).into(), Direction::South));
        }

        // 1 west edge <-> 4 west edge
        if *dir == Direction::West && col == edge_len && (0..edge_len).contains(&row) {
            let i = row;
            return Some(((edge_len * 3 - 1 - i, 0).into(), Direction::East));
        }

        if *dir == Direction::West && col == 0 && (edge_len * 2..edge_len * 3).contains(&row) {
            let i = row - edge_len * 2;
            return Some(((edge_len - 1 - i, edge_len).into(), Direction::East));
        }

        // 2 east edge <-> 5 east edge
        if *dir == Direction::East && col == edge_len * 3 - 1 && (0..edge_len).contains(&row) {
            let i = row;
            return Some((
                (edge_len * 3 - 1 - i, edge_len * 2 - 1).into(),
                Direction::West,
            ));
        }

        if *dir == Direction::East
            && col == edge_len * 2 - 1
            && (edge_len * 2..edge_len * 3).contains(&row)
        {
            let i = row - edge_len * 2;
            return Some(((edge_len - 1 - i, edge_len * 3 - 1).into(), Direction::West));
        }

        // 2 south edge <-> 3 east edge
        if *dir == Direction::South
            && row == edge_len - 1
            && (edge_len * 2..edge_len * 3).contains(&col)
        {
            let i = col - edge_len * 2;
            return Some(((edge_len + i, edge_len * 2 - 1).into(), Direction::West));
        }

        if *dir == Direction::East
            && col == edge_len * 2 - 1
            && (edge_len..edge_len * 2).contains(&row)
        {
            let i = row - edge_len;
            return Some(((edge_len - 1, edge_len * 2 + i).into(), Direction::North));
        }

        // 3 west edge <-> 4 north edge
        if *dir == Direction::West && col == edge_len && (edge_len..edge_len * 2).contains(&row) {
            let i = row - edge_len;
            return Some(((edge_len * 2, i).into(), Direction::South));
        }

        if *dir == Direction::North && row == edge_len * 2 && (0..edge_len).contains(&col) {
            let i = col;
            return Some(((edge_len + i, edge_len).into(), Direction::East));
        }

        // 5 south edge <-> 6 east edge
        if *dir == Direction::South
            && row == edge_len * 3 - 1
            && (edge_len..edge_len * 2).contains(&col)
        {
            let i = col - edge_len;
            return Some(((edge_len * 3 + i, edge_len - 1).into(), Direction::West));
        }

        if *dir == Direction::East
            && col == edge_len - 1
            && (edge_len * 3..edge_len * 4).contains(&row)
        {
            let i = row - edge_len * 3;
            return Some(((edge_len * 3 - 1, edge_len + i).into(), Direction::North));
        }

        None
    }

    fn traverse(&self, is_cube: bool) -> usize {
        let mut position = self.start;
        let mut dir = Direction::East;

        for instruction in &self.route {
            self.advance(&mut (*instruction).clone(), &mut position, &mut dir, is_cube);
            dir = dir.turn(&instruction.turn);
        }

        // turn back the last dummy turn
        dir = dir.turn(&Turn::Counterclockwise);

        1000 * (position.row() as usize + 1) + 4 * (position.col() as usize + 1) + dir.value()
    }

    fn advance(
        &self,
        instruction: &mut Instruction,
        position: &mut Coordinate,
        dir: &mut Direction,
        is_cube: bool,
    ) {
        let (row, col) = (position.row(), position.col());
        let tile = self.grid[*position];

        let Tile::Open {
            north: north_stop,
            south: south_stop,
            west: west_stop,
            east: east_stop
        } = tile else {
            // If we hit a wall, then we do nothing. This will only happen on boundary connections.
            return;
        };

        match dir {
            Direction::East => {
                if east_stop - col >= instruction.steps as isize {
                    position.1 += instruction.steps as isize;
                } else if east_stop < self.row_bounds[row as usize].1 as isize {
                    position.1 = east_stop;
                } else {
                    instruction.steps -= (east_stop - col + 1) as usize;
                    position.1 = east_stop;

                    let wrapped = if is_cube {
                        self.wrap_around_3d(position, dir)
                    } else {
                        self.wrap_around_2d(position, dir)
                    };

                    if let Some((new_position, new_dir)) = wrapped {
                        if self.grid[new_position].is_open() {
                            *position = new_position;
                            *dir = new_dir;
                            self.advance(instruction, position, dir, is_cube);
                        }
                    }
                }
            }
            Direction::South => {
                if south_stop - row >= instruction.steps as isize {
                    position.0 += instruction.steps as isize;
                } else if south_stop < self.col_bounds[col as usize].1 as isize {
                    position.0 = south_stop;
                } else {
                    instruction.steps -= (south_stop - row + 1) as usize;
                    position.0 = south_stop;

                    let wrapped = if is_cube {
                        self.wrap_around_3d(position, dir)
                    } else {
                        self.wrap_around_2d(position, dir)
                    };

                    if let Some((new_position, new_dir)) = wrapped {
                        if self.grid[new_position].is_open() {
                            *position = new_position;
                            *dir = new_dir;
                            self.advance(instruction, position, dir, is_cube);
                        }
                    }
                }
            }
            Direction::West => {
                if col - west_stop >= instruction.steps as isize {
                    position.1 -= instruction.steps as isize;
                } else if west_stop > self.row_bounds[row as usize].0 as isize {
                    position.1 = west_stop;
                } else {
                    instruction.steps -= (col - west_stop + 1) as usize;
                    position.1 = west_stop;

                    let wrapped = if is_cube {
                        self.wrap_around_3d(position, dir)
                    } else {
                        self.wrap_around_2d(position, dir)
                    };

                    if let Some((new_position, new_dir)) = wrapped {
                        if self.grid[new_position].is_open() {
                            *position = new_position;
                            *dir = new_dir;
                            self.advance(instruction, position, dir, is_cube);
                        }
                    }
                }
            }
            Direction::North => {
                if row - north_stop >= instruction.steps as isize {
                    position.0 -= instruction.steps as isize;
                } else if north_stop > self.col_bounds[col as usize].0 as isize {
                    position.0 = north_stop;
                } else {
                    instruction.steps -= (row - north_stop + 1) as usize;
                    position.0 = north_stop;

                    let wrapped = if is_cube {
                        self.wrap_around_3d(position, dir)
                    } else {
                        self.wrap_around_2d(position, dir)
                    };

                    if let Some((new_position, new_dir)) = wrapped {
                        if self.grid[new_position].is_open() {
                            *position = new_position;
                            *dir = new_dir;
                            self.advance(instruction, position, dir, is_cube);
                        }
                    }
                }
            }
        }
    }

}

impl TryFrom<&[String]> for Board {
    type Error = Error;

    fn try_from(lines: &[String]) -> Result<Self, Self::Error> {
        let mut chunks = lines.split(|l| l.is_empty());

        let grid_lines = chunks.next().ok_or_else(|| anyhow!("Invalid input"))?;
        let n = grid_lines.len();
        let m = grid_lines
            .iter()
            .map(|l| l.len())
            .max()
            .ok_or_else(|| anyhow!("Invalid input"))?;

        let mut grid = vec![vec![Tile::default(); m]; n];
        let mut wall_rows = vec![Vec::default(); n];
        let mut wall_cols = vec![Vec::default(); m];
        let mut row_bounds = vec![(0, m - 1); n];
        let mut col_bounds = vec![(0, n - 1); m];
        let mut start: Option<Coordinate> = None;

        for (i, l) in grid_lines.iter().enumerate() {
            row_bounds[i].1 = l.len() - 1;

            for (j, c) in l.char_indices() {
                let tile = Tile::try_from(c)?;

                if tile.is_wall() {
                    wall_rows[i].push(j);
                    wall_cols[j].push(i);
                }

                if start.is_none() && tile.is_open() {
                    start = Some((i, j).into());
                }

                grid[i][j] = tile;
            }
        }

        for i in 0..n {
            for j in 0..m {
                let tile = &grid[i][j];

                if j > 0 {
                    if grid[i][j - 1].is_void() && !tile.is_void() {
                        row_bounds[i].0 = j;
                    } else if !grid[i][j - 1].is_void() && tile.is_void() {
                        row_bounds[i].1 = j - 1;
                    }
                }

                if i > 0 {
                    if grid[i - 1][j].is_void() && !tile.is_void() {
                        col_bounds[j].0 = i;
                    } else if !grid[i - 1][j].is_void() && tile.is_void() {
                        col_bounds[j].1 = i - 1;
                    }
                }
            }
        }

        let route = chunks
            .next()
            .and_then(|lines| lines.get(0))
            .ok_or_else(|| anyhow!("Invalid input"))?
            .split_inclusive(&['L', 'R'])
            .map(|t| Instruction::from_str(t))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Board {
            grid: grid.into(),
            start: start.ok_or_else(|| anyhow!("Invalid input"))?,
            wall_rows,
            wall_cols,
            row_bounds,
            col_bounds,
            route,
        })
    }
}

pub fn parse_input(lines: &[String]) -> Result<Board> {
    let mut board: Board = lines.try_into()?;
    board.preprocess();
    Ok(board)
}

pub fn part_one(parsed: &Board) -> usize {
    parsed.traverse(false)
}

pub fn part_two(parsed: &Board) -> usize {
    parsed.traverse(true)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils;

    #[test]
    fn part_one_test() {
        let lines = utils::load_input("inputs/day_22.example").expect("could not load input");
        let parsed = parse_input(&lines).expect("could not parse input");
        assert_eq!(part_one(&parsed), 6032);
    }

    #[test]
    fn part_two_test() {
        // example has a different layout, so we use a custom example that follows the same layout
        // as the real input
        let lines = utils::load_input("inputs/day_22.custom").expect("could not load input");
        let parsed = parse_input(&lines).expect("could not parse input");
        assert_eq!(part_two(&parsed), 4028);
    }
}
