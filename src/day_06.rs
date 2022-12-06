use anyhow::{bail, Result};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Signal<'a>(&'a str);

impl Signal<'_> {
    fn start_of_marker(&self, unique_window_size: usize) -> usize {
        let mut last_index = [self.0.len(); 26];
        let mut start = 0;

        for (i, c) in self.0.char_indices() {
            let char_i = c as usize - 'a' as usize;

            if last_index[char_i] < self.0.len() && last_index[char_i] >= start {
                start = last_index[char_i] + 1
            }

            if i - start + 1 == unique_window_size {
                return i + 1;
            }

            last_index[char_i] = i
        }

        0
    }
}

pub fn parse_input(lines: &[String]) -> Result<Signal> {
    if lines.len() != 1 {
        bail!("Input should be exactly 1 line");
    }

    Ok(Signal(&lines[0]))
}

pub fn part_one(parsed: &Signal) -> usize {
    parsed.start_of_marker(4)
}

pub fn part_two(parsed: &Signal) -> usize {
    parsed.start_of_marker(14)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils;

    #[test]
    fn start_of_marker_test() {
        assert_eq!(
            Signal("mjqjpqmgbljsphdztnvjfqwrcgsmlb").start_of_marker(4),
            7
        );
        assert_eq!(Signal("bvwbjplbgvbhsrlpgdmjqwftvncz").start_of_marker(4), 5);
        assert_eq!(Signal("nppdvjthqldpwncqszvftbrmjlhg").start_of_marker(4), 6);
        assert_eq!(
            Signal("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg").start_of_marker(4),
            10
        );
        assert_eq!(
            Signal("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw").start_of_marker(4),
            11
        );
        assert_eq!(
            Signal("mjqjpqmgbljsphdztnvjfqwrcgsmlb").start_of_marker(14),
            19
        );
        assert_eq!(
            Signal("bvwbjplbgvbhsrlpgdmjqwftvncz").start_of_marker(14),
            23
        );
        assert_eq!(
            Signal("nppdvjthqldpwncqszvftbrmjlhg").start_of_marker(14),
            23
        );
        assert_eq!(
            Signal("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg").start_of_marker(14),
            29
        );
        assert_eq!(
            Signal("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw").start_of_marker(14),
            26
        );
    }

    #[test]
    fn part_one_test() {
        let lines = utils::load_input("inputs/day_06.example").expect("could not load input");
        let parsed = parse_input(&lines).expect("could not parse input");
        assert_eq!(part_one(&parsed), 7);
    }

    #[test]
    fn part_two_test() {
        let lines = utils::load_input("inputs/day_06.example").expect("could not load input");
        let parsed = parse_input(&lines).expect("could not parse input");
        assert_eq!(part_two(&parsed), 19);
    }
}
