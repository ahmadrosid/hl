#!/bin/bash
# set -e
cargo build -q

array=(
  "rules/css.yml"
  "rules/go.yml"
  "rules/java.yml"
  "rules/javascript.yml"
  "rules/typescript.yml"
  "rules/rust.yml"
)

for i in "${array[@]}"
do
	./target/debug/hl generate $i;
done

git status