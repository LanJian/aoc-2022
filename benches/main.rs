use criterion::criterion_main;

mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod day_09;
mod day_10;
mod day_11;
// ${BENCH_IMPORT_MARKER}

criterion_main! {
    day_01::benches,
    day_02::benches,
    day_03::benches,
    day_04::benches,
    day_05::benches,
    day_06::benches,
    day_07::benches,
    day_08::benches,
    day_09::benches,
    day_10::benches,
    day_11::benches,
    // ${CRITERION_MAIN_MARKER}
}
