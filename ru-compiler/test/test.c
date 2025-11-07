#include <stdio.h>
#include <stdlib.h>
extern unsigned long int foo(unsigned long int, unsigned long int);
int main(int argc, char** argv) {
    unsigned long a = 4, x = 10;
    printf("the result is %ld\n" ,foo(a, x));
    return 0;
}