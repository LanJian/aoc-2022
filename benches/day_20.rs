use aoc_2022::{day_20, utils};
use criterion::{black_box, criterion_group, Criterion};

fn benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("day 20: grove positioning system");

    group.bench_function("part 1", |b| {
        let lines = utils::load_input("inputs/day_20").expect("could not load input");
        let parsed = day_20::parse_input(&lines).expect("could not parse input");

        b.iter(|| day_20::part_one(black_box(&parsed)))
    });
    group.bench_function("part 2", |b| {
        let lines = utils::load_input("inputs/day_20").expect("could not load input");
        let parsed = day_20::parse_input(&lines).expect("could not parse input");

        b.iter(|| day_20::part_two(black_box(&parsed)))
    });
    group.bench_function("combined(including parsing)", |b| {
        let lines = utils::load_input("inputs/day_20").expect("could not load input");

        b.iter(|| {
            let parsed = day_20::parse_input(&lines).expect("could not parse input");
            day_20::part_one(black_box(&parsed));
            day_20::part_two(black_box(&parsed));
        })
    });
    group.finish();
}

criterion_group!(benches, benchmark);
