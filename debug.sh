#!/usr/bin/env bash
# set -e
clear
cargo run -p generator -- -i rules/bash.yml -o crates/core
cargo fmt

cargo run --package hl --example html crates/tests/src/test/testdata/input/bash.sh.stub -l bash > table.html

# Copy result
# cp table.html crates/tests/src/test/testdata/output/v.html.stub
# cp test-file crates/tests/src/test/testdata/input/v.v.stub
cargo test --workspace -p hl_tests
# git status
