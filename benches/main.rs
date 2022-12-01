use criterion::criterion_main;

mod day_01;
// ${BENCH_IMPORT_MARKER}

criterion_main! {
    day_01::benches,
    // ${CRITERION_MAIN_MARKER}
}
