#!/bin/bash

# Name of your compiled parser
EXECUTABLE="./first-compiler"

for input_file in test*.exp; do
    output_file1="${input_file%.exp}.astdag"
    output_file2="${input_file%.exp}.ll"
    $EXECUTABLE "$input_file" > "$output_file1"
    mv first.ll $output_file2
done

echo " All tests completed."

