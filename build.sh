#!/bin/bash
set -e
# cargo build -q --workspace -p generator

array=(
  "rules/actionscript.yml"
  "rules/bash.yml"
  "rules/c.yml"
  "rules/clojure.yml"
  "rules/css.yml"
  "rules/cuda.yml"
  "rules/dart.yml"
  "rules/edn.yml"
  "rules/erlang.yml"
  "rules/cpp.yml"
  "rules/cs.yml"
  "rules/go.yml"
  "rules/groovy.yml"
  "rules/haskell.yml"
  "rules/html.yml"
  "rules/java.yml"
  "rules/javascript.yml"
  "rules/json.yml"
  "rules/kotlin.yml"
  "rules/lua.yml"
  "rules/makefile.yml"
  "rules/markdown.yml"
  "rules/nim.yml"
  "rules/php.yml"
  "rules/python.yml"
  "rules/raw.yml"
  "rules/ruby.yml"
  "rules/rust.yml"
  "rules/toml.yml"
  "rules/typescript.yml"
  "rules/vue.yml"
  "rules/yaml.yml"
  "rules/zig.yml"
)

for i in "${array[@]}"
do
	./target/debug/hl-gen -i $i -o crates/core;
done

cargo fmt
cargo test --workspace -p hl_tests
git status
