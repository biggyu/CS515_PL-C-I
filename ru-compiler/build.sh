#!/bin/bash
cd trivial-ssa 
cargo build --release
cp target/release/ru-compiler ../ru-compiler
