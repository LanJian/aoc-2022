use aoc_2022::{day_21, solution::Solution, utils};

fn main() {
    let lines = utils::load_input("inputs/day_21").expect("could not load input");
    let parsed = day_21::parse_input(&lines).expect("could not parse input");
    let solution = Solution {
        part_one: day_21::part_one(&parsed),
        part_two: day_21::part_two(&parsed),
    };
    println!("{}", solution);
}
