use aoc_2022::{day_03, solution::Solution, utils};

fn main() {
    let lines = utils::load_input("inputs/day_03").expect("could not load input");
    let parsed = day_03::parse_input(&lines).expect("could not parse input");
    let solution = Solution {
        part_one: day_03::part_one(&parsed),
        part_two: day_03::part_two(&parsed),
    };
    println!("{}", solution);
}
