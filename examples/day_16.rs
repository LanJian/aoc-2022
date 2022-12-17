use aoc_2022::{day_16, solution::Solution, utils};

fn main() {
    let lines = utils::load_input("inputs/day_16").expect("could not load input");
    let parsed = day_16::parse_input(&lines).expect("could not parse input");
    let solution = Solution {
        part_one: day_16::part_one(&parsed),
        part_two: day_16::part_two(&parsed),
    };
    println!("{}", solution);
}
