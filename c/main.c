#include <stdlib.h>

#include "device.h"

int main(void) {
    // Allocate a new device pointer 
    Device *d = new_device();
    
    // TODO main loop here
    
    // Cleanup
    free(d);
}