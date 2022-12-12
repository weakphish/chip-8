#include "instructions.h"

void CLS(Device *d)
{
    for (int x = 0; x < DISPLAY_WIDTH; x++)
    {
        for (int y = 0; y < DISPLAY_HEIGHT; y++)
        {
            d->display[x][y] = 0;
        }
    }
}

void RET(Device *d)
{
    d->stack_pointer--;
    d->program_counter = d->stack[d->stack_pointer];
}
