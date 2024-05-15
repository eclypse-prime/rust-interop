#include <stdio.h>

#include "c_interop.h"

int main(int argc, char** argv)
{
    rs_print();
    printf("%d\n", rs_add(1, 2));
}