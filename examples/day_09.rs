use aoc_2022::{day_09, solution::Solution, utils};

fn main() {
    let lines = utils::load_input("inputs/day_09").expect("could not load input");
    let parsed = day_09::parse_input(&lines).expect("could not parse input");
    let solution = Solution {
        part_one: day_09::part_one(&parsed),
        part_two: day_09::part_two(&parsed),
    };
    println!("{}", solution);
}
