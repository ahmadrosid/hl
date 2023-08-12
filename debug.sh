#!/usr/bin/env bash
# set -e
clear
cargo run -p generator -- -i rules/vue.yml -o crates/core
cargo fmt

cargo run --package hl --example html test-file -l vue > table.html

# Copy result
# cp table.html crates/tests/src/test/testdata/output/v.html.stub
# cp test-file crates/tests/src/test/testdata/input/v.v.stub
# cargo test --workspace -p hl_tests
# git status
