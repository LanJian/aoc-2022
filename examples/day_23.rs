use aoc_2022::{day_23, solution::Solution, utils};

fn main() {
    let lines = utils::load_input("inputs/day_23").expect("could not load input");
    let parsed = day_23::parse_input(&lines).expect("could not parse input");
    let solution = Solution {
        part_one: day_23::part_one(&parsed),
        part_two: day_23::part_two(&parsed),
    };
    println!("{}", solution);
}
