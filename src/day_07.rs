use anyhow::{anyhow, Error, Result};
use rustc_hash::FxHashMap;
use std::{cell::RefCell, rc::Rc, str::FromStr};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum OutputLine {
    Cd(String),
    Ls,
    Dir(String),
    File(File),
}

impl FromStr for OutputLine {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens = s.split_whitespace();
        let ret = if s.starts_with("$ cd") {
            Self::Cd(
                tokens
                    .skip(2)
                    .next()
                    .ok_or_else(|| anyhow!("Invalid cd output line"))?
                    .to_owned(),
            )
        } else if s.starts_with("$ ls") {
            Self::Ls
        } else if s.starts_with("dir") {
            Self::Dir(
                tokens
                    .skip(1)
                    .next()
                    .ok_or_else(|| anyhow!("Invalid dir output line"))?
                    .to_owned(),
            )
        } else {
            Self::File(File::from_str(s)?)
        };

        Ok(ret)
    }
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct File {
    size: usize,
}

impl FromStr for File {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            size: s
                .split_whitespace()
                .next()
                .ok_or_else(|| anyhow!("Invalid file output line"))?
                .parse::<usize>()?,
        })
    }
}

#[derive(Debug, Clone, Default, Eq, PartialEq)]
struct Dir {
    size: usize,
    dirs: FxHashMap<String, Rc<RefCell<Dir>>>,
    files: FxHashMap<String, File>,
}

type DirWrapper = Rc<RefCell<Dir>>;

impl Dir {
    fn build(output_lines: &Vec<OutputLine>) -> DirWrapper {
        let root = Rc::new(RefCell::new(Dir::default()));
        let mut cur = Rc::clone(&root);
        let mut stack = vec![];

        for line in output_lines {
            match line {
                OutputLine::Cd(name) if name == "/" => {
                    cur = Rc::clone(&root);
                }
                OutputLine::Cd(name) if name == ".." => {
                    if let Some(p) = stack.pop() {
                        cur = p;
                    }
                }
                OutputLine::Cd(name) => {
                    let new_dir = Dir {
                        size: 0,
                        dirs: FxHashMap::default(),
                        files: FxHashMap::default(),
                    };
                    cur.borrow_mut()
                        .dirs
                        .entry(name.to_owned())
                        .or_insert(Rc::new(RefCell::new(new_dir)));
                    let new_cur = Rc::clone(&cur.borrow().dirs[name]);
                    stack.push(Rc::clone(&cur));
                    cur = new_cur;
                }
                OutputLine::Ls => {}
                OutputLine::Dir(name) => {
                    let new_dir = Dir {
                        size: 0,
                        dirs: FxHashMap::default(),
                        files: FxHashMap::default(),
                    };
                    cur.borrow_mut()
                        .dirs
                        .entry(name.to_owned())
                        .or_insert(Rc::new(RefCell::new(new_dir)));
                }
                OutputLine::File(file) => cur.borrow_mut().size += file.size,
            }

        }

        root
    }

    fn calculate_size(&mut self) -> usize {
        let result: usize = self
            .dirs
            .iter()
            .map(|(_, v)| v.borrow_mut().calculate_size())
            .sum();

        self.size += result;
        self.size
    }

    fn sum_dir_sizes(&self, max_size: usize) -> usize {
        let sum = if self.size <= max_size { self.size } else { 0 };

        sum + self
            .dirs
            .iter()
            .map(|(_, v)| v.borrow().sum_dir_sizes(max_size))
            .sum::<usize>()
    }

    fn min_freeable_size(&self, desired: usize) -> usize {
        let result = self.dirs
            .iter()
            .map(|(_, v)| v.borrow().min_freeable_size(desired))
            .filter(|&s| s >= desired)
            .min()
            .unwrap_or(0);

        if self.size < desired {
            result
        } else if result < desired {
            self.size
        } else {
            self.size.min(result)
        }
    }
}

pub fn parse_input(lines: &[String]) -> Result<Vec<OutputLine>> {
    lines.iter().map(|l| OutputLine::from_str(l)).collect()
}

pub fn part_one(parsed: &Vec<OutputLine>) -> usize {
    let dir = Dir::build(parsed);
    dir.borrow_mut().calculate_size();
    let ret = dir.borrow().sum_dir_sizes(100000);
    ret
}

pub fn part_two(parsed: &Vec<OutputLine>) -> usize {
    let dir = Dir::build(parsed);
    dir.borrow_mut().calculate_size();
    let desired = 30000000 - (70000000 - dir.borrow().size);
    let ret = dir.borrow().min_freeable_size(desired);
    ret
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils;

    #[test]
    fn output_line_parse_test() {
        assert_eq!(
            OutputLine::from_str("$ cd a").unwrap(),
            OutputLine::Cd("a".to_owned())
        );
        assert_eq!(OutputLine::from_str("$ ls").unwrap(), OutputLine::Ls);
        assert_eq!(
            OutputLine::from_str("dir a").unwrap(),
            OutputLine::Dir("a".to_owned())
        );
        assert_eq!(
            OutputLine::from_str("14848514 b.txt").unwrap(),
            OutputLine::File(File { size: 14848514 })
        );
    }

    #[test]
    fn part_one_test() {
        let lines = utils::load_input("inputs/day_07.example").expect("could not load input");
        let parsed = parse_input(&lines).expect("could not parse input");
        assert_eq!(part_one(&parsed), 95437);
    }

    #[test]
    fn part_two_test() {
        let lines = utils::load_input("inputs/day_07.example").expect("could not load input");
        let parsed = parse_input(&lines).expect("could not parse input");
        assert_eq!(part_two(&parsed), 24933642);
    }
}
