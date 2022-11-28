new day:
  echo "creating scaffold for day {{day}}"
  touch ./inputs/day_{{day}}
  touch ./inputs/day_{{day}}.example
  cp ./templates/src/day_xx.rs.tpl ./src/day_{{day}}.rs
  cp ./templates/examples/day_xx.rs.tpl ./examples/day_{{day}}.rs
  cp ./templates/benches/day_xx.rs.tpl ./benches/day_{{day}}.rs
  sed -i 's/day_xx/day_{{day}}/g' \
    ./src/day_{{day}}.rs ./examples/day_{{day}}.rs ./benches/day_{{day}}.rs
  sed -i 's/day xx/day {{day}}/g' ./benches/day_{{day}}.rs
  sed -i \
    's/\/\/ ${LIB_IMPORT_MARKER}/pub mod day_{{day}};\n\/\/ ${LIB_IMPORT_MARKER}/g' \
    ./src/lib.rs
  sed -i \
    's/\/\/ ${BENCH_IMPORT_MARKER}/mod day_{{day}};\n\/\/ ${BENCH_IMPORT_MARKER}/g' \
    ./benches/main.rs
  sed -i \
    's/    \/\/ ${CRITERION_MAIN_MARKER}/    day_{{day}}::benches,\n    \/\/ ${CRITERION_MAIN_MARKER}/g' \
    ./benches/main.rs
