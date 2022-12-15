#include <stdlib.h>
#include <time.h>

#include "device.h"

Device *new_device()
{
    Device *d = malloc(sizeof(Device));
    d->program_counter = START_ADDRESS;

    // Load font into memory
    for (unsigned int i = 0; i < FONTSET_SIZE; ++i)
    {
        d->memory[FONTSET_START_ADDRESS + i] = fontset[i];
    }

    return d;
}

int rand_int()
{
    srand(time(NULL));
    return rand();
}