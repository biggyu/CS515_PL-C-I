#include <stdio.h>
#include <stdlib.h>
extern unsigned long int foo(unsigned long int);
int main(int argc, char** argv) {
    unsigned long var = 4;
    printf("the result is %ld\n" ,foo(var));
    return 0;
}