#!/bin/bash

tests=(
  "10,20"
  "20,30"
  "257,23"
  "319,199"
  "320,200"
)

echo "Compiling"
cargo build --bin calculator --release

echo "Testing"
for test in ${tests[@]}; do
  echo "Testing with $test"
  result=$(./target/release/calculator ${test//,/ })
  echo "${result}"

  comparison=$(./plotpixel.rb ${test//,/ })
  echo "${comparison}"

  if [ "${result}" != "${comparison}" ]; then
    echo "Eek!"
    exit 1
  else
    echo "OK"
  fi
done
