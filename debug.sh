#!/usr/bin/env bash
# set -e
clear
cargo run -p generator -- -i rules/javascript.yml -o crates/core
cargo fmt

# Copy result
#cp table.html crates/tests/src/test/testdata/output/javascript.html.stub
#cp test-file crates/tests/src/test/testdata/input/javascript.js.stub
# cargo test --workspace -p hl_tests
git status
