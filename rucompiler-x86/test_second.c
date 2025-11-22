#include<stdio.h>
#include<stdlib.h>
extern unsigned long int foo (unsigned long, unsigned long);
int main(int arg, char** argv) {
    unsigned long x = 10, a = 8;
    printf("the result is %lu\n", foo(a, x));
    return 0;
}
