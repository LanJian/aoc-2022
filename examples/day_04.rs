use aoc_2022::{day_04, utils};

fn main() {
    let lines = utils::load_input("inputs/day_04").expect("could not load input");
    let parsed = day_04::parse_input(&lines).expect("could not parse input");

    println!("part 1: {}", day_04::part_one(&parsed));
    println!("part 2: {}", day_04::part_two(&parsed));
}
