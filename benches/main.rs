use criterion::criterion_main;

mod day_01;
mod day_02;
mod day_03;
// ${BENCH_IMPORT_MARKER}

criterion_main! {
    day_01::benches,
    day_02::benches,
    day_03::benches,
    // ${CRITERION_MAIN_MARKER}
}
