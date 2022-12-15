#ifndef DEVICE_H
#define DEVICE_H

#include <stdint.h>

#define KEYS 16
#define DISPLAY_HEIGHT 32
#define DISPLAY_WIDTH 64
#define NUM_REGISTERS 16
#define START_ADDRESS 0x200 // where a ROM starts in memory
#define STACK_SIZE 16
#define MEM_BYTES 4096

#define FONTSET_SIZE 80
#define FONTSET_START_ADDRESS 0x50

uint8_t fontset[FONTSET_SIZE] = {
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80  // F
};

typedef uint16_t opcode;

typedef struct DeviceStruct
{
  // The current opcode
  unsigned char opcode;
  /*
   * The CHIP-8 has sixteen 8-bit registers, labeled V0 to VF. Each register is
   * able to hold any value from 0x00 to 0xFF. Register VF is a bit special.
   * It’s used as a flag to hold information about the result of operations.
   */
  unsigned char registers[NUM_REGISTERS];
  // Store memory addresses for use in operations
  uint16_t index_register;
  // Holds the address of the next instruction to execute
  uint16_t program_counter;
  // Decrement at a rate of 60hz, unless zero
  unsigned char delay_timer;
  // Same as above, but buzz a tone when non-zero
  unsigned char sound_timer;
  // Points to top of the stack
  unsigned char stack_pointer;
  // The CHIP-8 has 4096 bytes of memory, meaning the address space is from
  // 0x000 to 0xFFF.
  unsigned char memory[MEM_BYTES];
  // Stack
  uint16_t stack[STACK_SIZE];
  // Keypad
  unsigned char key[KEYS];
  // Display
  uint32_t display[DISPLAY_WIDTH][DISPLAY_HEIGHT];
} Device;

/**
 * Allocate a device and return a pointer to it.
 *
 * @return A pointer to the allocated device.
 */
Device *new_device();

/**
 * Load a rom into memory
 *
 * @param d A pointer to the Chip-8 Device struct
 * @param filename The name of the ROM file
 */
void load_rom(Device *d, char *filename);

/**
 * Push a value to the stack.
 *
 * @param d A pointer to the device struct.
 * @param val The value to push to the stack
 */
void push(Device *d, uint16_t val);

/**
 * Pop a value off the stack, leaving it in place. Simply decrement the stack
 * pointer.
 *
 * @param d A pointer to the device struct.
 * @return a copy of the value at SP
 */
uint16_t pop(Device *d);

/**
 * Generate a random number.
 *
 * @return a random integer
 */
int rand_int();

#endif
