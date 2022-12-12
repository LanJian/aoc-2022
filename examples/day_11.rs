use aoc_2022::{day_11, solution::Solution, utils};

fn main() {
    let lines = utils::load_input("inputs/day_11").expect("could not load input");
    let parsed = day_11::parse_input(&lines).expect("could not parse input");
    let solution = Solution {
        part_one: day_11::part_one(&parsed),
        part_two: day_11::part_two(&parsed),
    };
    println!("{}", solution);
}
