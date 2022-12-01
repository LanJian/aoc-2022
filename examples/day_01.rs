use aoc_2022::{day_01, utils};

fn main() {
    let lines = utils::load_input("inputs/day_01").expect("could not load input");
    let parsed = day_01::parse_input(lines).expect("could not parse input");

    println!("part 1: {}", day_01::part_one(&parsed));
    println!("part 2: {}", day_01::part_two(&parsed));
}
