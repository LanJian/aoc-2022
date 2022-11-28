use std::{convert::TryFrom, ops::{Index, IndexMut}};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Coordinate(i64, i64);

impl From<(usize, usize)> for Coordinate {
    fn from(coords: (usize, usize)) -> Self {
        Coordinate(coords.0 as i64, coords.1 as i64)
    }
}

impl From<(i64, i64)> for Coordinate {
    fn from(coords: (i64, i64)) -> Self {
        Coordinate(coords.0, coords.1)
    }
}

impl Coordinate {
    pub fn x(&self) -> i64 {
        self.1
    }

    pub fn y(&self) -> i64 {
        self.0
    }

    pub fn row(&self) -> i64 {
        self.0
    }

    pub fn col(&self) -> i64 {
        self.1
    }

    pub fn north(&self) -> Self {
        Self(self.0 - 1, self.1)
    }

    pub fn south(&self) -> Self {
        Self(self.0 + 1, self.1)
    }

    pub fn east(&self) -> Self {
        Self(self.0, self.1 + 1)
    }

    pub fn west(&self) -> Self {
        Self(self.0, self.1 - 1)
    }

    pub fn northeast(&self) -> Self {
        self.north().east()
    }

    pub fn northwest(&self) -> Self {
        self.north().west()
    }

    pub fn southeast(&self) -> Self {
        self.south().east()
    }

    pub fn southwest(&self) -> Self {
        self.south().west()
    }

    pub fn cardinal_neighbours(&self) -> [Self; 4] {
        [
            self.north(),
            self.south(),
            self.east(),
            self.west(),
        ]
    }

    pub fn ordinal_neighbours(&self) -> [Self; 4] {
        [
            self.northeast(),
            self.northwest(),
            self.southeast(),
            self.southwest(),
        ]
    }

    pub fn neighbours(&self) -> [Self; 8] {
        [
            self.north(),
            self.south(),
            self.east(),
            self.west(),
            self.northeast(),
            self.northwest(),
            self.southeast(),
            self.southwest(),
        ]
    }
}


#[derive(Debug, Clone)]
pub struct Grid<T>
{
    grid: Vec<Vec<T>>,
    pub n: usize,
    pub m: usize,
}

impl<T> TryFrom<Vec<String>> for Grid<T> where T: TryFrom<char> {
    type Error = T::Error;

    fn try_from(lines: Vec<String>) -> Result<Self, Self::Error> {
        let grid = lines
            .iter()
            .map(|line| {
                line.chars()
                    .map(|c| T::try_from(c))
                    .collect::<Result<Vec<T>, T::Error>>()
            })
            .collect::<Result<Vec<Vec<T>>, T::Error>>()?;

        Ok(grid.into())
    }
}

impl<T> From<Vec<Vec<T>>> for Grid<T> {
    fn from(grid: Vec<Vec<T>>) -> Self {
        let n = grid.len();
        let m = grid[0].len();
        Self { grid, n, m }
    }
}

impl<T> Index<Coordinate> for Grid<T> {
    type Output = T;

    fn index(&self, idx: Coordinate) -> &Self::Output {
        &self.grid[idx.0 as usize][idx.1 as usize]
    }
}

impl<T> IndexMut<Coordinate> for Grid<T> {
    fn index_mut(&mut self, idx: Coordinate) -> &mut Self::Output {
        &mut self.grid[idx.0 as usize][idx.1 as usize]
    }
}

impl<T> Grid<T> where T: Copy {
    pub fn new(n: usize, m: usize, default: T) -> Self {
        Self {
            grid: vec![vec![default; m]; n],
            n,
            m,
        }
    }

    pub fn is_in_bounds(&self, coord: Coordinate) -> bool {
        (0..self.n as i64).contains(&coord.0) && (0..self.m as i64).contains(&coord.1)
    }
}
