#include "instructions.h"

void (*decode(unsigned int opcode))(Device *d) {
  unsigned int nibble_1 = (opcode & 0xFF00); // FIXME I think?
}

void CLS(Device *d) {
  for (int x = 0; x < DISPLAY_WIDTH; x++) {
    for (int y = 0; y < DISPLAY_HEIGHT; y++) {
      d->display[x][y] = 0;
    }
  }
}

void RET(Device *d) {
  d->stack_pointer--;
  d->program_counter = d->stack[d->stack_pointer];
}

void JMP_ADDR(Device *d) { d->program_counter = (d->opcode & 0x0FFF) >> 8u; }

void LD_VX_BYTE(Device *d) {
  unsigned int x = (d->opcode & 0x0F00u) >> 8u;
  unsigned int byte = (d->opcode & 0x00FFu);

  d->registers[x] = byte;
}

void ADD_VX_BYTE(Device *d) {
  unsigned int x = (d->opcode & 0x0F00u) >> 8u;
  unsigned int byte = (d->opcode & 0x00FFu);

  d->registers[x] += byte;
}

void LDI_ADDR(Device *d) {
  unsigned int nnn = (d->opcode & 0x0FFFu);
  d->index_register = nnn;
}

void DRW(Device *d);