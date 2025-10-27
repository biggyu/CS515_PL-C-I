#!/bin/bash

# Name of your compiled parser
EXECUTABLE="./ru-compiler"

for input_file in test*.exp; do
    output_file="${input_file%.exp}.astdag"
    $EXECUTABLE "$input_file" > "$output_file"
done

echo " All tests completed."

