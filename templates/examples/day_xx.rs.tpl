use aoc_2022::{day_xx, solution::Solution, utils};

fn main() {
    let lines = utils::load_input("inputs/day_xx").expect("could not load input");
    let parsed = day_xx::parse_input(&lines).expect("could not parse input");
    let solution = Solution {
        part_one: day_xx::part_one(&parsed),
        part_two: day_xx::part_two(&parsed),
    };
    println!("{}", solution);
}
