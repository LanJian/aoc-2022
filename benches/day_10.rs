use aoc_2022::{day_10, utils};
use criterion::{black_box, criterion_group, Criterion};

fn benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("day 10: cathode-ray tube");

    group.bench_function("combined(including parsing, both parts are solved together)", |b| {
        let lines = utils::load_input("inputs/day_10").expect("could not load input");

        b.iter(|| {
            let parsed = day_10::parse_input(&lines).expect("could not parse input");
            day_10::part_one(black_box(&parsed));
            day_10::part_two(black_box(&parsed));
        })
    });
    group.finish();
}

criterion_group!(benches, benchmark);
