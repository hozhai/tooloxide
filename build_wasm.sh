#!/usr/bin/env bash

for file in rust/*; do
  if [[ "$(basename "$file")" == "util.rs" ]]; then
    continue
  fi
  cd "$file" && wasm-pack build --target web --out-dir ../../src/lib/pkg/$(basename "${file%.*}")
  cd ../..
done