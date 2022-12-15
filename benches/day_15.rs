use aoc_2022::{day_15, utils};
use criterion::{black_box, criterion_group, Criterion};

fn benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("day 15: beacon exclusion zone");

    group.bench_function("part 1", |b| {
        let lines = utils::load_input("inputs/day_15").expect("could not load input");
        let parsed = day_15::parse_input(&lines).expect("could not parse input");

        b.iter(|| day_15::part_one(black_box(&parsed)))
    });
    group.bench_function("part 2", |b| {
        let lines = utils::load_input("inputs/day_15").expect("could not load input");
        let parsed = day_15::parse_input(&lines).expect("could not parse input");

        b.iter(|| day_15::part_two(black_box(&parsed)))
    });
    group.bench_function("combined(including parsing)", |b| {
        let lines = utils::load_input("inputs/day_15").expect("could not load input");

        b.iter(|| {
            let parsed = day_15::parse_input(&lines).expect("could not parse input");
            day_15::part_one(black_box(&parsed));
            day_15::part_two(black_box(&parsed));
        })
    });
    group.finish();
}

criterion_group!(benches, benchmark);
