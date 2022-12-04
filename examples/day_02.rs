use aoc_2022::{day_02, solution::Solution, utils};

fn main() {
    let lines = utils::load_input("inputs/day_02").expect("could not load input");
    let parsed = day_02::parse_input(lines).expect("could not parse input");
    let solution = Solution {
        part_one: day_02::part_one(&parsed),
        part_two: day_02::part_two(&parsed),
    };
    println!("{}", solution);
}
