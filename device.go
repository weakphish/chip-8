package main

const STACK_SIZE = 16

type Device struct {
	ram     *RAM
	cpu     *CPU
	display *Display
	stack   *Stack
}

func newDevice() *Device {
	// doing this inelegantly so I can construct the Stack with a reference to the CPU
	cpu := newCPU()
	stack := newStack(&cpu.stackPointer)
	return &Device{
		ram:     newRAM(),
		cpu:     cpu,
		display: newDisplay(),
		stack:   stack,
	}
}

// This function represents a single cycle in the emulation loop.
func (d *Device) emulateCycle() {
	// Get the instruction at the PC and increment the PC
	var instruction uint16 = uint16(d.ram.memory[d.cpu.programCounter])
	d.cpu.programCounter++

	// Decode instruction
	switch instruction {
	case 0x00E0:
		// CLS
		d.clearScreen()
	}
}

func (d *Device) clearScreen() {
	for i := 0; i < DISPLAY_HEIGHT; i++ {
		for j := 0; j < DISPLAY_WIDTH; j++ {
			d.display.frame[i][j] = false
		}
	}
}
