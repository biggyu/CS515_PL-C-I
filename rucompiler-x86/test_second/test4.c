#include <stdio.h>
#include <stdlib.h>
extern unsigned long int foo(unsigned long int, unsigned long int);
int main(int argc, char** argv) {
    unsigned long a = 10, b = 8;
    printf("the result is %ld\n" ,foo(b, a));
    return 0;
}