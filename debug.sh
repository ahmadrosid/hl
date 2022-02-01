#!/usr/bin/env bash
#source ./build.sh
cargo run generate "rules/zig.yml"
#cargo run src/test/testdata/input/bash.sh.stub -l bash > table.html
#cargo run src/test/testdata/input/bash.sh.stub -l haskell > table.html
cargo run test-file -l zig > table.html
cargo fmt
