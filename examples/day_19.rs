use aoc_2022::{day_19, solution::Solution, utils};

fn main() {
    let lines = utils::load_input("inputs/day_19").expect("could not load input");
    let parsed = day_19::parse_input(&lines).expect("could not parse input");
    let solution = Solution {
        part_one: day_19::part_one(&parsed),
        part_two: day_19::part_two(&parsed),
    };
    println!("{}", solution);
}
