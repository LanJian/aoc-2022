use anyhow::Result;

pub fn parse_input(lines: &[String]) -> Result<Vec<_>> {
    todo!()
}

pub fn part_one(parsed: &Vec<_>) -> usize {
    todo!()
}

pub fn part_two(parsed: &Vec<_>) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::utils;
    use super::*;

    #[test]
    fn part_one_test() {
        let lines = utils::load_input("inputs/day_xx.example").expect("could not load input");
        let parsed = parse_input(&lines).expect("could not parse input");
        assert_eq!(part_one(&parsed), 0);
    }

    #[test]
    fn part_two_test() {
        let lines = utils::load_input("inputs/day_xx.example").expect("could not load input");
        let parsed = parse_input(&lines).expect("could not parse input");
        assert_eq!(part_two(&parsed), 0);
    }
}
