use aoc_2022::{day_25, utils};
use criterion::{black_box, criterion_group, Criterion};

fn benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("day 25: full of hot air");

    group.bench_function("part 1", |b| {
        let lines = utils::load_input("inputs/day_25").expect("could not load input");
        let parsed = day_25::parse_input(&lines).expect("could not parse input");

        b.iter(|| day_25::part_one(black_box(&parsed)))
    });
    group.bench_function("part 2", |b| {
        let lines = utils::load_input("inputs/day_25").expect("could not load input");
        let parsed = day_25::parse_input(&lines).expect("could not parse input");

        b.iter(|| day_25::part_two(black_box(&parsed)))
    });
    group.bench_function("combined(including parsing)", |b| {
        let lines = utils::load_input("inputs/day_25").expect("could not load input");

        b.iter(|| {
            let parsed = day_25::parse_input(&lines).expect("could not parse input");
            day_25::part_one(black_box(&parsed));
            day_25::part_two(black_box(&parsed));
        })
    });
    group.finish();
}

criterion_group!(benches, benchmark);
