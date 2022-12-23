use aoc_2022::{day_23, utils};
use criterion::{black_box, criterion_group, Criterion};

fn benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("day 23: unstable diffusion");

    group.bench_function("part 1", |b| {
        let lines = utils::load_input("inputs/day_23").expect("could not load input");
        let parsed = day_23::parse_input(&lines).expect("could not parse input");

        b.iter(|| day_23::part_one(black_box(&parsed)))
    });
    group.bench_function("part 2", |b| {
        let lines = utils::load_input("inputs/day_23").expect("could not load input");
        let parsed = day_23::parse_input(&lines).expect("could not parse input");

        b.iter(|| day_23::part_two(black_box(&parsed)))
    });
    group.bench_function("combined(including parsing)", |b| {
        let lines = utils::load_input("inputs/day_23").expect("could not load input");

        b.iter(|| {
            let parsed = day_23::parse_input(&lines).expect("could not parse input");
            day_23::part_one(black_box(&parsed));
            day_23::part_two(black_box(&parsed));
        })
    });
    group.finish();
}

criterion_group!(benches, benchmark);
