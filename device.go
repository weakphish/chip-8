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

// This function represents a single cycle in the emulation loop. It will get the instruction
// at the program counter, decode and execute it, and then make the neccesary updates to the state
// of the device.
func (d *Device) emulateCycle() {
	// Get the opcode at the PC and increment the PC
	opcode := uint16(d.ram.mem[d.cpu.programCounter])
	d.cpu.programCounter++

	nibble := (opcode & 0xF000) >> 12
	x := uint8((opcode & 0x0F00) >> 8)
	y := uint8((opcode & 0x00F0) >> 4)
	n := uint8((opcode & 0x000F))
	nn := uint8(opcode & 0x00FF)
	nnn := opcode & 0x0FFF

	// Decode instruction
	switch nibble {
	case 0x00:
		switch nn {
		case 0xE0:
			d.clearScreen()
		case 0xEE:
			d.returnFrom()
		}
	case 0x1:
		d.jump(nnn)
	case 0x6:
		d.add(x, nn)
	case 0x7:
		d.add(x, nn)
	case 0xA:
		d.setIndex(nnn)
	case 0xD:
		d.draw(x, y, n)
	}
}

// CLS - 00E0
// Clear the screen of the display (set all the pixels to 'off')
func (d *Device) clearScreen() {
	for i := 0; i < DISPLAY_HEIGHT; i++ {
		for j := 0; j < DISPLAY_WIDTH; j++ {
			d.display.frame[i][j] = false
		}
	}
}

// RET - 00EE
// Return from a subroutine. Pops the value at the top of the stack
// (indicated by the stack pointer SP) and puts it in PC.
func (d *Device) returnFrom() {
	d.cpu.programCounter = d.stack.pop()
}

// JMP — 1NNN
// Jump to the address in NNN. Sets the PC to NNN.
func (d *Device) jump(nnn uint16) {
	d.cpu.programCounter = nnn
}

// SET - 6XNN
// Simply set the register VX to the value NN.
func (d *Device) setRegister(vx, nn uint8) {
	d.cpu.generalRegisters[vx] = nn
}

// ADD - 7XNN
// Add the value NN to the value of register VX and store the result in VX.
func (d *Device) add(x, nn uint8) {
	// VX := VX + NN
	d.cpu.generalRegisters[x] += uint8(nn)
}

// SET I - ANNN
// Set index register I to the value of NNN
func (d *Device) setIndex(nnn uint16) {
	d.cpu.indexRegister = nnn
}

// DRAW - DXYN
// Draw the display
func (d *Device) draw(x, y, n uint8) {
	xCoord := d.cpu.generalRegisters[x] % DISPLAY_WIDTH
	yCoord := d.cpu.generalRegisters[y] % DISPLAY_WIDTH
}
