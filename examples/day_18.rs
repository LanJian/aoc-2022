use aoc_2022::{day_18, solution::Solution, utils};

fn main() {
    let lines = utils::load_input("inputs/day_18").expect("could not load input");
    let parsed = day_18::parse_input(&lines).expect("could not parse input");
    let solution = Solution {
        part_one: day_18::part_one(&parsed),
        part_two: day_18::part_two(&parsed),
    };
    println!("{}", solution);
}
