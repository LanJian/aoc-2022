use aoc_2022::{day_03, utils};
use criterion::{black_box, criterion_group, Criterion};

fn benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("day 03: rucksack reorganization");

    group.bench_function("part 1", |b| {
        let lines = utils::load_input("inputs/day_03").expect("could not load input");
        let parsed = day_03::parse_input(lines).expect("could not parse input");

        b.iter(|| black_box(day_03::part_one(&parsed)))
    });
    group.bench_function("part 2", |b| {
        let lines = utils::load_input("inputs/day_03").expect("could not load input");
        let parsed = day_03::parse_input(lines).expect("could not parse input");

        b.iter(|| black_box(day_03::part_two(&parsed)))
    });
    group.bench_function("combined(including parsing)", |b| {
        b.iter(|| {
            let lines = utils::load_input("inputs/day_03").expect("could not load input");
            let parsed = day_03::parse_input(lines).expect("could not parse input");
            black_box(day_03::part_one(&parsed));
            black_box(day_03::part_two(&parsed));
        })
    });
    group.finish();
}

criterion_group!(benches, benchmark);
