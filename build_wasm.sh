#!/usr/bin/env bash

for file in rust/*; do
  wasm-pack build --target web --out-dir pkg/${$file%.*} $file
done