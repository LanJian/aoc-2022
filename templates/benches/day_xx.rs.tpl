use aoc_2022::{
    utils, day_xx,
};
use criterion::{black_box, criterion_group, Criterion};

fn benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("day xx: REPLACE ME");

    group.bench_function("part 1", |b| {
        let lines = utils::load_input("inputs/day_xx").expect("could not load input");
        let _ = day_xx::parse_input(lines).expect("could not parse input");

        b.iter(|| {
            black_box(day_xx::part_one(&_))
        })
    });
    group.bench_function("part 2", |b| {
        let lines = utils::load_input("inputs/day_xx").expect("could not load input");
        let _ = day_xx::parse_input(lines).expect("could not parse input");

        b.iter(|| {
            black_box(day_xx::part_two(&_))
        })
    });
    group.finish();
}

criterion_group!(benches, benchmark);
