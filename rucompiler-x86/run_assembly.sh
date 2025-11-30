#!/bin/bash

sh build_second.sh
gcc test_second.c first.s -o test.out
./test.out