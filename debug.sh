#!/usr/bin/env bash
source ./build.sh
cargo run -- src/test/testdata/input/yaml.yml.stub -l yaml > table.html