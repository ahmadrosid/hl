#!/bin/bash
set -e
cargo build -q

array=(
  "rules/bash.yml"
  "rules/c.yml"
  "rules/css.yml"
  "rules/cpp.yml"
  "rules/go.yml"
  "rules/html.yml"
  "rules/java.yml"
  "rules/javascript.yml"
  "rules/lua.yml"
  "rules/php.yml"
  "rules/python.yml"
  "rules/rust.yml"
  "rules/typescript.yml"
  "rules/yaml.yml"
)

for i in "${array[@]}"
do
	./target/debug/hl generate $i;
done

cargo test
git status