use aoc_2022::{day_14, solution::Solution, utils};

fn main() {
    let lines = utils::load_input("inputs/day_14").expect("could not load input");
    let parsed = day_14::parse_input(&lines).expect("could not parse input");
    let solution = Solution {
        part_one: day_14::part_one(&parsed),
        part_two: day_14::part_two(&parsed),
    };
    println!("{}", solution);
}
