use aoc_2022::{day_03, utils};

fn main() {
    let lines = utils::load_input("inputs/day_03").expect("could not load input");
    let parsed = day_03::parse_input(lines).expect("could not parse input");

    println!("part 1: {}", day_03::part_one(&parsed));
    println!("part 2: {}", day_03::part_two(&parsed));
}
