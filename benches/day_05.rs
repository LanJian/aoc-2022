use aoc_2022::{day_05, utils};
use criterion::{black_box, criterion_group, Criterion};

fn benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("day 05: supply stacks");

    group.bench_function("part 1", |b| {
        let lines = utils::load_input("inputs/day_05").expect("could not load input");
        let parsed = day_05::parse_input(&lines).expect("could not parse input");

        b.iter(|| day_05::part_one(black_box(&mut parsed.clone())))
    });
    group.bench_function("part 2", |b| {
        let lines = utils::load_input("inputs/day_05").expect("could not load input");
        let parsed = day_05::parse_input(&lines).expect("could not parse input");

        b.iter(|| day_05::part_two(black_box(&mut parsed.clone())))
    });
    group.bench_function("combined(including parsing)", |b| {
        let lines = utils::load_input("inputs/day_05").expect("could not load input");

        b.iter(|| {
            let parsed = day_05::parse_input(&lines).expect("could not parse input");
            day_05::part_one(black_box(&mut parsed.clone()));
            day_05::part_two(black_box(&mut parsed.clone()));
        })
    });
    group.finish();
}

criterion_group!(benches, benchmark);
