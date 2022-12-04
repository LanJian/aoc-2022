use aoc_2022::{day_04, solution::Solution, utils};

fn main() {
    let lines = utils::load_input("inputs/day_04").expect("could not load input");
    let parsed = day_04::parse_input(&lines).expect("could not parse input");
    let solution = Solution {
        part_one: day_04::part_one(&parsed),
        part_two: day_04::part_two(&parsed),
    };
    println!("{}", solution);
}
