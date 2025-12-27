#!/bin/bash

# Build script for compiling Rust code to WebAssembly

set -e

echo "Building for WASM..."
cargo build --target wasm32-unknown-unknown --release

echo "Generating WASM bindings..."
wasm-bindgen --target web --out-dir ./pkg ./target/wasm32-unknown-unknown/release/rs_wasm_experiment.wasm

echo "Build complete! Output files are in ./pkg/"
ls -lh ./pkg/

