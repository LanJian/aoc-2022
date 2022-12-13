use aoc_2022::{day_13, solution::Solution, utils};

fn main() {
    let lines = utils::load_input("inputs/day_13").expect("could not load input");
    let parsed = day_13::parse_input(&lines).expect("could not parse input");
    let solution = Solution {
        part_one: day_13::part_one(&parsed),
        part_two: day_13::part_two(&parsed),
    };
    println!("{}", solution);
}
