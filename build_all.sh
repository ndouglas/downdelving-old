#!/bin/bash
cargo build;
cargo build --release;
./build_wasm.sh;
