use aoc_2022::{day_xx::{parse_input, self}, utils};

fn main() {
    let lines = utils::load_input("inputs/day_xx").expect("could not load input");
    let _ = parse_input(lines).expect("could not parse input");

    println!("part 1: {}", day_xx::part_one(&_));
    println!("part 2: {}", day_xx::part_two(&_));
}
