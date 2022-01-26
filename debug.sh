#!/usr/bin/env bash
#source ./build.sh
#cargo run -- src/test/testdata/input/haskell.hs.stub -l haskell > table.html
cargo run -- test-file.md -l md > table.html
#cargo run -- generate rules/markdown.yml