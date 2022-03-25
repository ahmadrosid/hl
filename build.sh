#!/bin/bash
# set -e
cargo build -q --workspace -p generator
find rules/ -type f -name "*.yml" | xargs -I % sh -c './target/debug/hl-gen -i % -o crates/core;'
cargo fmt
cargo test --workspace -p hl_tests
git status
