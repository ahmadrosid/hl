#!/usr/bin/env bash
#source ./build.sh
#cargo run src/test/testdata/input/haskell.hs.stub -l haskell > table.html
cargo run test-file -l ruby > table.html
#cargo run generate rules/markdown.yml
#cargo run generate "rules/ruby.yml"
