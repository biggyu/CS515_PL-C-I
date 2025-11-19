#!/bin/bash

# Name of your compiled parser
EXECUTABLE="./rucompiler-x86-first"
TEST_DIR="test_first"

for input_file in "$TEST_DIR"/test*.exp; do
    # output_file="${input_file%.exp}.s"
    $EXECUTABLE "$input_file"
done

echo " All tests completed."

