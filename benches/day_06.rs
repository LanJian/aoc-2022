use aoc_2022::{day_06, utils};
use criterion::{black_box, criterion_group, Criterion};

fn benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("day 06: tuning trouble");

    group.bench_function("part 1", |b| {
        let lines = utils::load_input("inputs/day_06").expect("could not load input");
        let parsed = day_06::parse_input(&lines).expect("could not parse input");

        b.iter(|| day_06::part_one(black_box(&parsed)))
    });
    group.bench_function("part 2", |b| {
        let lines = utils::load_input("inputs/day_06").expect("could not load input");
        let parsed = day_06::parse_input(&lines).expect("could not parse input");

        b.iter(|| day_06::part_two(black_box(&parsed)))
    });
    group.bench_function("combined(including parsing)", |b| {
        let lines = utils::load_input("inputs/day_06").expect("could not load input");

        b.iter(|| {
            let parsed = day_06::parse_input(&lines).expect("could not parse input");
            day_06::part_one(black_box(&parsed));
            day_06::part_two(black_box(&parsed));
        })
    });
    group.finish();
}

criterion_group!(benches, benchmark);
