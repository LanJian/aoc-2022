use aoc_2022::{day_12, solution::Solution, utils};

fn main() {
    let lines = utils::load_input("inputs/day_12").expect("could not load input");
    let parsed = day_12::parse_input(&lines).expect("could not parse input");
    let solution = Solution {
        part_one: day_12::part_one(&parsed),
        part_two: day_12::part_two(&parsed),
    };
    println!("{}", solution);
}
