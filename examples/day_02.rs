use aoc_2022::{day_02, utils};

fn main() {
    let lines = utils::load_input("inputs/day_02").expect("could not load input");
    let parsed = day_02::parse_input(lines).expect("could not parse input");

    println!("part 1: {}", day_02::part_one(&parsed));
    println!("part 2: {}", day_02::part_two(&parsed));
}
