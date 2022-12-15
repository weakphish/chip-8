#include <stdio.h>
#include <stdlib.h>

#include "device.h"

int main(int argc, char *argv[]) {
    // Allocate a new device pointer 
    Device *d = new_device();

    // Read ROM file
    FILE *ptr;
    ptr = fopen(argv[1], "rb");  // r for read, b for binary
    fread(buffer, sizeof(buffer), 1, ptr); // read 10 bytes to our buffer

    // TODO main loop here
    
    // Cleanup
    free(d);
}