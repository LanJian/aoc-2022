use aoc_2022::{day_20, solution::Solution, utils};

fn main() {
    let lines = utils::load_input("inputs/day_20").expect("could not load input");
    let parsed = day_20::parse_input(&lines).expect("could not parse input");
    let solution = Solution {
        part_one: day_20::part_one(&parsed),
        part_two: day_20::part_two(&parsed),
    };
    println!("{}", solution);
}
