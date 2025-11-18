#!/bin/bash
cd first
cargo build --release
cp target/release/rucompiler-x86-first ../rucompiler-x86-first
cd ../
./rucompiler-x86-first first.exp 