use anyhow::{bail, Result};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Signal<'a>(&'a str);

impl Signal<'_> {
    fn start_of_packet(&self) -> usize {
        let mut chars = ['.'; 4];
        let mut j = 0;

        for (i, c) in self.0.char_indices() {
            chars[j] = c;

            if i < 3
                || chars[0] == chars[1]
                || chars[0] == chars[2]
                || chars[0] == chars[3]
                || chars[1] == chars[2]
                || chars[1] == chars[3]
                || chars[2] == chars[3]
            {
                j = (j + 1) % 4
            } else {
                return i + 1;
            }
        }

        0
    }

    fn start_of_message(&self) -> usize {
        let chars: Vec<char> = self.0.chars().collect();
        let mut acc = [0_usize; 26];

        for i in 0..chars.len() {
            let c = chars[i];
            let char_i = c as usize - 'a' as usize;
            acc[char_i] += 1;

            if i < 13 {
                continue;
            }

            if i >= 14 {
                let left_char = chars[i - 14];
                let left_char_i = left_char as usize - 'a' as usize;
                acc[left_char_i] -= 1;
            }

            if acc.iter().all(|&x| x < 2) {
                return i + 1;
            }
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
    parsed.start_of_packet()
}

pub fn part_two(parsed: &Signal) -> usize {
    parsed.start_of_message()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils;

    #[test]
    fn start_of_packet_test() {
        assert_eq!(
            Signal("mjqjpqmgbljsphdztnvjfqwrcgsmlb").start_of_packet(),
            7
        );
        assert_eq!(Signal("bvwbjplbgvbhsrlpgdmjqwftvncz").start_of_packet(), 5);
        assert_eq!(Signal("nppdvjthqldpwncqszvftbrmjlhg").start_of_packet(), 6);
        assert_eq!(
            Signal("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg").start_of_packet(),
            10
        );
        assert_eq!(
            Signal("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw").start_of_packet(),
            11
        );
    }

    #[test]
    fn start_of_message_test() {
        assert_eq!(
            Signal("mjqjpqmgbljsphdztnvjfqwrcgsmlb").start_of_message(),
            19
        );
        assert_eq!(
            Signal("bvwbjplbgvbhsrlpgdmjqwftvncz").start_of_message(),
            23
        );
        assert_eq!(
            Signal("nppdvjthqldpwncqszvftbrmjlhg").start_of_message(),
            23
        );
        assert_eq!(
            Signal("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg").start_of_message(),
            29
        );
        assert_eq!(
            Signal("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw").start_of_message(),
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
