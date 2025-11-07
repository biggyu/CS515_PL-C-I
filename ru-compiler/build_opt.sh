#!/bin/bash
cd opt-ssa 
cargo build --release
cp target/release/ru-compiler ../ru-compiler
