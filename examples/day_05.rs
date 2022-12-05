use aoc_2022::{day_05, solution::Solution, utils};

fn main() {
    let lines = utils::load_input("inputs/day_05").expect("could not load input");
    let parsed = day_05::parse_input(&lines).expect("could not parse input");
    let solution = Solution {
        part_one: day_05::part_one(&mut parsed.clone()),
        part_two: day_05::part_two(&mut parsed.clone()),
    };
    println!("{}", solution);
}
