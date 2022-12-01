use aoc_2022::{day_01, utils};
use criterion::{black_box, criterion_group, Criterion};

fn benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("day 01: calorie counting");

    group.bench_function("part 1", |b| {
        let lines = utils::load_input("inputs/day_01").expect("could not load input");
        let parsed = day_01::parse_input(lines).expect("could not parse input");

        b.iter(|| black_box(day_01::part_one(&parsed)))
    });
    group.bench_function("part 2", |b| {
        let lines = utils::load_input("inputs/day_01").expect("could not load input");
        let parsed = day_01::parse_input(lines).expect("could not parse input");

        b.iter(|| black_box(day_01::part_two(&parsed)))
    });
    group.finish();
}

criterion_group!(benches, benchmark);
