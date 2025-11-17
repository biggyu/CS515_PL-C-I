#!/bin/bash

# Name of your compiled parser
EXECUTABLE="./first-compiler"
TEST_DIR="test"

for input_file in "$TEST_DIR"/test*.exp; do
    output_file="${input_file%.exp}.astdag"
    $EXECUTABLE "$input_file" > "$output_file"
done

echo " All tests completed."

