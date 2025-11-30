#include <stdio.h>
#include <stdlib.h>
extern unsigned long int foo(unsigned long int);
// extern unsigned long int foo();
int main(int argc, char** argv) {
    unsigned long a = 2;
    printf("the result is %ld\n" ,foo(a));
    return 0;
}