use aoc_2022::{day_07, solution::Solution, utils};

fn main() {
    let lines = utils::load_input("inputs/day_07").expect("could not load input");
    let parsed = day_07::parse_input(&lines).expect("could not parse input");
    let solution = Solution {
        part_one: day_07::part_one(&parsed),
        part_two: day_07::part_two(&parsed),
    };
    println!("{}", solution);
}
