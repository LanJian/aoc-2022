use aoc_2022::{day_11, utils};
use criterion::{black_box, criterion_group, Criterion};

fn benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("day 11: monkey in the middle");

    group.bench_function("part 1", |b| {
        let lines = utils::load_input("inputs/day_11").expect("could not load input");
        let parsed = day_11::parse_input(&lines).expect("could not parse input");

        b.iter(|| day_11::part_one(black_box(&parsed)))
    });
    group.bench_function("part 2", |b| {
        let lines = utils::load_input("inputs/day_11").expect("could not load input");
        let parsed = day_11::parse_input(&lines).expect("could not parse input");

        b.iter(|| day_11::part_two(black_box(&parsed)))
    });
    group.bench_function("combined(including parsing)", |b| {
        let lines = utils::load_input("inputs/day_11").expect("could not load input");

        b.iter(|| {
            let parsed = day_11::parse_input(&lines).expect("could not parse input");
            day_11::part_one(black_box(&parsed));
            day_11::part_two(black_box(&parsed));
        })
    });
    group.finish();
}

criterion_group!(benches, benchmark);
