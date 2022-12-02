use criterion::criterion_main;

mod day_01;
mod day_02;
// ${BENCH_IMPORT_MARKER}

criterion_main! {
    day_01::benches,
    day_02::benches,
    // ${CRITERION_MAIN_MARKER}
}
