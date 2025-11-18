#!/bin/bash
cd second
cargo build --release
cp target/release/rucompiler-x86-second ../rucompiler-x86-second
cd ../
./rucompiler-x86-second first.rucomp