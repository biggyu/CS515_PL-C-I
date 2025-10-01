#!/bin/bash

# Name of your compiled parser
EXECUTABLE="./scanparse"

for input_file in test*.input; do
    output_file="${input_file%.input}.output"
    $EXECUTABLE "$input_file" > "$output_file"
done

echo " All tests completed."
