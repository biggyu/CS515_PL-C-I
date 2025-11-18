#!/bin/bash

# Name of your compiled parser
EXECUTABLE="./ru-compiler"
TEST_DIR="test"

for input_file in "$TEST_DIR"/test*.rucomp; do
    base_name="${input_file%.rucomp}"
    ll_file="${base_name}.ll"
    obj_file="${base_name}.o"
    out_file="${base_name}.out"
    c_file="${base_name}.c"
    # ll_file="$TEST_DIR/${base_name}.ll"
    # obj_file="$TEST_DIR/${base_name}.o"
    # out_file="$TEST_DIR/${base_name}.out"
    # c_file="$TEST_DIR/${base_name}.c"

    $EXECUTABLE "$input_file"
    # $EXECUTABLE "$input_file" > "$ll_file"
    clang -c "$ll_file" -o "$obj_file"
    clang "$c_file" "$obj_file" -o "$out_file"
done

echo " All tests completed."

