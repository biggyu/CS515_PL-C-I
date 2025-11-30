#include <stdio.h>
#include <stdlib.h>
extern unsigned long int foo(unsigned long int, unsigned long int, unsigned long int, unsigned long int, unsigned long int, unsigned long int);
int main(int argc, char** argv) {
    unsigned long a = 0, b = 7, c = 2, d = 2, e = 3, f = 4;
    printf("the result is %ld\n" ,foo(a, b, c, d, e, f));
    return 0;
}