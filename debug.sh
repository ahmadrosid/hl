#!/usr/bin/env bash
clear
#source ./build.sh
cargo run -p generator -- -i rules/html.yml -o crates/core
#cargo run --package hl --example html test-file -l vue > table.html
#cargo run --package hl --example html test-file -l vue
#cargo fmt

# Copy result
#cp table.html src/test/testdata/output/javascript.html.stub
#cp test-file src/test/testdata/input/javascript.js.stub
#cargo test