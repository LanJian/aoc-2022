use aoc_2022::{day_01, solution::Solution, utils};

fn main() {
    let lines = utils::load_input("inputs/day_01").expect("could not load input");
    let parsed = day_01::parse_input(lines).expect("could not parse input");
    let solution = Solution {
        part_one: day_01::part_one(&parsed),
        part_two: day_01::part_two(&parsed),
    };
    println!("{}", solution);
}
