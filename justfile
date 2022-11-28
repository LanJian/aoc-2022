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
