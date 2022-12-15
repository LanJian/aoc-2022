use aoc_2022::{day_15, solution::Solution, utils};

fn main() {
    let lines = utils::load_input("inputs/day_15").expect("could not load input");
    let parsed = day_15::parse_input(&lines).expect("could not parse input");
    let solution = Solution {
        part_one: day_15::part_one(&parsed),
        part_two: day_15::part_two(&parsed),
    };
    println!("{}", solution);
}
