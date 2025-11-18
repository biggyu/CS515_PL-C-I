#include<stdio.h>
#include<stdlib.h>
extern unsigned long int foo (unsigned long int, unsigned long, unsigned long);
int main(int arg, char** argv) {
    unsigned long int b = 2, c = 10, e = 8, result = 0;
    result = foo(c, e, b);
    printf("the result is %lu\n", result);
    return 0;
}