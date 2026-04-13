#include <stdio.h>

#include "config.h"

int main(void) {
    printf("Hello, %s\\n", config.name);
    Config cfg;

    return 0;
}
