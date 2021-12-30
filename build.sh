#!/bin/bash
cargo build -q

array=(
  "rules/c.yml"
  "rules/css.yml"
  "rules/cpp.yml"
  "rules/go.yml"
  "rules/java.yml"
  "rules/javascript.yml"
  "rules/rust.yml"
  "rules/typescript.yml"
)

for i in "${array[@]}"
do
	./target/debug/hl generate $i;
done

git status