use aoc_2022::{day_10, solution::Solution, utils};

fn main() {
    let lines = utils::load_input("inputs/day_10").expect("could not load input");
    let parsed = day_10::parse_input(&lines).expect("could not parse input");
    let solution = Solution {
        part_one: day_10::part_one(&parsed),
        part_two: day_10::part_two(&parsed),
    };
    println!("{}", solution);
}
