use aoc_2022::{day_08, solution::Solution, utils};

fn main() {
    let lines = utils::load_input("inputs/day_08").expect("could not load input");
    let parsed = day_08::parse_input(&lines).expect("could not parse input");
    let solution = Solution {
        part_one: day_08::part_one(&parsed),
        part_two: day_08::part_two(&parsed),
    };
    println!("{}", solution);
}
