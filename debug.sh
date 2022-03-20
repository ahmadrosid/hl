#!/usr/bin/env bash
#source ./build.sh
#cargo run generate "rules/typescript.yml"
#cargo run src/test/testdata/input/bash.sh.stub -l bash > table.html
#cargo run src/test/testdata/input/bash.sh.stub -l haskell > table.html
#cargo run test-file -l vue
cargo run test-file -l vue > table.html
cargo fmt

# Copy result
#cp table.html src/test/testdata/output/javascript.html.stub
#cp test-file src/test/testdata/input/javascript.js.stub
#cargo test