#!/usr/bin/env bash
source ./build.sh
#cargo run src/test/testdata/input/haskell.hs.stub -l haskell > table.html
cargo run test-file -l yaml > table.html
#cargo run generate rules/markdown.yml