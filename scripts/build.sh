#!/bin/bash

array=(
  "rules/css.yml"
  "rules/go.yml"
  "rules/javascript.yml"
  "rules/rust.yml"
)
for i in "${array[@]}"
do
	./target/debug/hl generate $i;
done