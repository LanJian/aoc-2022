use aoc_2022::{day_25, solution::Solution, utils};

fn main() {
    let lines = utils::load_input("inputs/day_25").expect("could not load input");
    let parsed = day_25::parse_input(&lines).expect("could not parse input");
    let solution = Solution {
        part_one: day_25::part_one(&parsed),
        part_two: day_25::part_two(&parsed),
    };
    println!("{}", solution);
}
