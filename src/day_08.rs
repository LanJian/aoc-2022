use anyhow::Result;

use crate::grid::Grid;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Visibility {
    Unknown,
    Visible,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq)]
struct ViewingDistances {
    left: usize,
    right: usize,
    up: usize,
    down: usize,
}

impl ViewingDistances {
    fn scenic_score(&self) -> usize {
        self.left * self.right * self.up * self.down
    }
}

type Grove = Grid<u8>;

impl Grove {
    fn trees_visible(&self) -> usize {
        if self.n < 2 || self.m < 2 {
            return self.n * self.m;
        }

        // now we know its at least 2x2
        let mut count = 0;
        let mut vis_grid = Grid::new(self.n, self.m, Visibility::Unknown);

        let mut top_down_maxes = vec![0; self.m];
        let mut bottom_up_maxes = vec![0; self.m];

        for i in 0..self.n {
            let mut left_max = 0;
            let mut right_max = 0;

            for j in 0..self.m {
                let left_coord = (i, j).into();
                let left = self[left_coord];

                let right_coord = (i, self.m - j - 1).into();
                let right = self[right_coord];

                let bottom_coord = (self.n - i - 1, j).into();
                let bottom = self[bottom_coord];

                if vis_grid[left_coord] != Visibility::Visible
                    && (self.is_on_edge(left_coord) || left > left_max || left > top_down_maxes[j])
                {
                    vis_grid[left_coord] = Visibility::Visible;
                    count += 1;
                }

                if vis_grid[right_coord] != Visibility::Visible
                    && (self.is_on_edge(right_coord) || right > right_max)
                {
                    vis_grid[right_coord] = Visibility::Visible;
                    count += 1;
                }

                if vis_grid[bottom_coord] != Visibility::Visible
                    && (self.is_on_edge(bottom_coord) || bottom > bottom_up_maxes[j])
                {
                    vis_grid[bottom_coord] = Visibility::Visible;
                    count += 1;
                }

                if left > left_max {
                    left_max = left;
                }

                if right > right_max {
                    right_max = right;
                }

                if left > top_down_maxes[j] {
                    top_down_maxes[j] = left;
                }

                if bottom > bottom_up_maxes[j] {
                    bottom_up_maxes[j] = bottom;
                }
            }
        }

        count
    }

    fn max_scenic_score(&self) -> usize {
        let mut vd_grid = Grid::new(self.n, self.m, ViewingDistances::default());

        // left to right
        for i in 0..self.n {
            let mut stack = Vec::with_capacity(self.m);

            for j in 0..self.m {
                let coord = (i, j).into();
                let h = self[coord];

                while let Some(&k) = stack.last() {
                    if self[(i, k).into()] < h {
                        stack.pop();
                    } else {
                        break;
                    }
                }
                vd_grid[coord].left = j - stack.last().map(|k| *k).unwrap_or(0);
                stack.push(j);
            }
        }

        // right to left
        for i in 0..self.n {
            let mut stack = Vec::with_capacity(self.m);

            for j in 0..self.m {
                let jj = self.m - j - 1;
                let coord = (i, jj).into();
                let h = self[coord];

                while let Some(&k) = stack.last() {
                    let kk = self.m - k - 1;
                    if self[(i, kk).into()] < h {
                        stack.pop();
                    } else {
                        break;
                    }
                }
                vd_grid[coord].right = j - stack.last().map(|k| *k).unwrap_or(0);
                stack.push(j);
            }
        }

        // top to bottom
        for j in 0..self.m {
            let mut stack = Vec::with_capacity(self.n);

            for i in 0..self.n {
                let coord = (i, j).into();
                let h = self[coord];

                while let Some(&k) = stack.last() {
                    if self[(k, j).into()] < h {
                        stack.pop();
                    } else {
                        break;
                    }
                }
                vd_grid[coord].up = i - stack.last().map(|k| *k).unwrap_or(0);
                stack.push(i);
            }
        }

        // bottom to top
        for j in 0..self.m {
            let mut stack = Vec::with_capacity(self.n);

            for i in 0..self.n {
                let ii = self.n - i - 1;
                let coord = (ii, j).into();
                let h = self[coord];

                while let Some(&k) = stack.last() {
                    let kk = self.n - k - 1;
                    if self[(kk, j).into()] < h {
                        stack.pop();
                    } else {
                        break;
                    }
                }
                vd_grid[coord].down = i - stack.last().map(|k| *k).unwrap_or(0);
                stack.push(i);
            }
        }

        let mut max = 0;
        for i in 0..self.n {
            for j in 0..self.m {
                let score = vd_grid[(i, j).into()].scenic_score();
                if score > max {
                    max = score;
                }
            }
        }

        max
    }
}

pub fn parse_input(lines: &[String]) -> Result<Grove> {
    Ok(Grove::try_from(lines)?)
}

pub fn part_one(parsed: &Grove) -> usize {
    parsed.trees_visible()
}

pub fn part_two(parsed: &Grove) -> usize {
    parsed.max_scenic_score()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils;

    #[test]
    fn part_one_test() {
        let lines = utils::load_input("inputs/day_08.example").expect("could not load input");
        let parsed = parse_input(&lines).expect("could not parse input");
        assert_eq!(part_one(&parsed), 21);
    }

    #[test]
    fn part_two_test() {
        let lines = utils::load_input("inputs/day_08.example").expect("could not load input");
        let parsed = parse_input(&lines).expect("could not parse input");
        assert_eq!(part_two(&parsed), 8);
    }
}
