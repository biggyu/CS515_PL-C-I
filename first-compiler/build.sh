#!/bin/bash
cd pa2
cargo build --release
cp target/release/first-compiler ../first-compiler
