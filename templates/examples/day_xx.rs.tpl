use aoc_2022::{day_xx, utils};

fn main() {
    let lines = utils::load_input("inputs/day_xx").expect("could not load input");
    let parsed = day_xx::parse_input(&lines).expect("could not parse input");

    println!("part 1: {}", day_xx::part_one(&parsed));
    println!("part 2: {}", day_xx::part_two(&parsed));
}
