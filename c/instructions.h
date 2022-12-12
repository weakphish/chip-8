#ifndef INSTRUCT_H
#define INSTRUCT_H

#include "device.h"

/** 00E0 - Clear the display. Set the entire video buffer to zeroes. */
void CLS(Device *d);

/**
 * 00EE - Return from a subroutine.
 * The top of the stack has the address of one instruction past the one that called the subroutine,
 * so we can put that back into the PC.
 */
void RET(Device *d);

/**
 * 1NNN - Jump to location NNN, no stack interaction required
 * (Set PC to NNN)
 */
void JMP_ADDR(Device *d);

/**
 * 2NNN - Call a subroutine at NNN
 * Put current program counter at the top of the stack,
 * so that we can return eventually.
 */
void CLL_ADDR(Device *d);

/**
 * 3XKK - Skip next instruction if vX == kk
 */
void SE_VX_BYTE(Device *d);

/**
 * 4XKK - Skip next instruction if vX != kk
 */
void SNE_VX_BYTE(Device *d);

/**
 * 5XY0 - Skip next instruction if vX = vY
 */
void SE_VX_VY(Device *d);

/**
 * 6XKK - Set vX = kk
 */
void LD_VX_BYTE(Device *d);

/**
 * 7XKK Set vX = vX + kk
 */
void ADD_VX_BYTE(Device *d);

/**
 * 8XY0 Set vX = vY
 */
void LD_VX_VY(Device *d);


/**
 * 8XY1 Set vX = vX OR vY
 */
void OR_VX_VY(Device *d);


/**
 * Set vX = vX AND vY
 */
void AND_8XY2(Device *d);


/**
 * Set vX = vX XOR vY
 */
void XOR_8XY3(Device *d);


/**
 * Set vX = vX = vY, vF = carry
 * "The values of Vx and Vy are added together. 
 * If the result is greater than 8 bits (i.e., > 255,) 
 * VF is set to 1, otherwise 0. Only the lowest 8 bits 
 * of the result are kept, and stored in Vx."
 */
void ADD_8XY4(Device *d);


/**
 * Set vX = vX - vY, set VF = NOT borrow
 * "If Vx > Vy, then VF is set to 1, otherwise 0. Then Vy is subtracted from Vx, 
 * and the results stored in Vx."
 */
void SUB_8XY5(Device *d);


/**
 * Set vX = vX SHR 1 (SHR = shift right)
 * "If the least-significant bit of Vx is 1, then VF is set to 1, otherwise 0.
 * Then Vx is divided by 2."
 */
void SHR_8XY(Device *d);

/**
 * Set vX = vY - vX, set VF = NOT borrow
 * If vY > vX, then vF is set to 1, otherwise 0.
 * Then, vX is subtracted from vY, and results stored in vX.
 */
void SUBN_8XY7(Device *d);

/**
 * Set vX = vX SHL 1.
 * If the most significant bit of vX is 1, thenn vF is set to 1,
 * otherwise 0. Then, vX is multiplied by 2.
 */
void SHL_8XYE(Device *d);

/**
 * Skip the next instruction if vX != vY.
 */
void SNE_9XY0(Device *d);

/**
 * Set I (index register) = nnn
 */
void LDI_ANNN(Device *d);

/**
 * Jump to the location nnn + v0
 */
void JMP_BNNN(Device *d);

/**
 * Set vX = random byte AND kk
 */
void RND_CXKK(Device *d);

/**
 * Display an n-byte sprite starting at the memory location I at 
 * (vX, vY), set vF = collision
 */
void DRW_DXYN(Device *d);

/**
 * Skip next instruction if key with the value of vX is pressed.
 */
void SKP_EX9E(Device *d);

/**
 * Skip next instruction if the key with the value of vX is NOT pressed.
 */
void SKNP_EXA1(Device *d);

/**
 * Set vX = value of the delay timer
 */
void LDDT_FX07(Device *d);

/**
 * Wait for a key press, store value of the key in vX
 */
void LDK_FX0A(Device *d);

/**
 * Set delay timer = vX
 */
void LDDT_FX15(Device *d);


/**
 * Set sound timer = vX
 */
void LDST_FX18(Device *d);

/**
 * Set I = I + vX
 * @param difjdf
 * @return
 */
void ADD_FX1E(Device *d);

/**
 * FX29 - Set I = Location of sprite for digit vX
 */
void LD_F_VX(Device *d);

/**
 * FX33 - Store BCD representation of vX in memory locations I, I+1, I+2
 * Take decimal value of vX, and places the hundres digit in memory
 * at location I, tens in I+1, ones in I+2.
 */
void LD_B_VX(Device *d);

/**
 * FX55 - Store registers v0 through vX in memory starting at location I.
 */
void LD_I_VX(Device *d);

/**
 * FX65 - Read registers v0 through vX from memory, starting at location I.
 */

#endif