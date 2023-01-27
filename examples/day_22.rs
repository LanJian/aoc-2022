use aoc_2022::{day_22, solution::Solution, utils};

fn main() {
    let lines = utils::load_input("inputs/day_22").expect("could not load input");
    let parsed = day_22::parse_input(&lines).expect("could not parse input");
    let solution = Solution {
        part_one: day_22::part_one(&parsed),
        part_two: day_22::part_two(&parsed),
    };
    println!("{}", solution);
}
