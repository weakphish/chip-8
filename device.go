package main

import (
	"image/color"
	"log"
	"os"

	"github.com/hajimehoshi/ebiten"
)

const (
	MEM_BYTES      = 4096
	PROG_MEM_START = 0x200
	FONT_COUNT     = 80
)

var font_set = [FONT_COUNT]byte{
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
	0xF0, 0x80, 0xF0, 0x80, 0x80, // F
}

const (
	DISPLAY_HEIGHT = 32
	DISPLAY_WIDTH  = 64
)

const STACK_SIZE = 16

type Device struct {
	ram     [MEM_BYTES]byte
	cpu     *CPU
	display [DISPLAY_HEIGHT][DISPLAY_WIDTH]bool
	stack   *Stack
}

func newDevice() *Device {
	// doing this inelegantly so I can construct the Stack with a reference to the CPU
	cpu := newCPU()
	stack := newStack(&cpu.stackPointer)
	return &Device{
		cpu:   cpu,
		stack: stack,
	}
}

func (d *Device) LoadROM(rom *os.File) {
	_, err := rom.Read(d.ram[PROG_MEM_START:])
	if err != nil {
		panic(err)
	}
}

func (d *Device) LoadFont() {
	for i, f := range font_set {
		d.ram[i] = f
	}
}

func (d *Device) Layout(outsideWidth, outsideHeight int) (screenWidth, screenHeight int) {
	return DISPLAY_WIDTH, DISPLAY_HEIGHT
}

func (d *Device) Draw(screen *ebiten.Image) {
	for x := range d.display {
		for y := range d.display[x] {
			screen.Set(x, y, color.White)
		}
	}
}

// This function represents a single cycle in the emulation loop. It will get the instruction
// at the program counter, decode and execute it, and then make the neccesary updates to the state
// of the device.
func (d *Device) Update(screen *ebiten.Image) error {
	// Get the opcode at the PC and increment the PC
	var opcode uint16
	b1 := d.ram[d.cpu.programCounter]
	d.cpu.programCounter++
	b2 := d.ram[d.cpu.programCounter]
	d.cpu.programCounter++

	opcode = (uint16(b1) << 8) | uint16(b2)

	log.Printf("Current Opcode: %x\n", opcode)

	nibble := (opcode & 0xF000) >> 12
	x := uint8((opcode & 0x0F00) >> 8)
	y := uint8((opcode & 0x00F0) >> 4)
	n := uint8((opcode & 0x000F))
	nn := uint8(opcode & 0x00FF)
	nnn := opcode & 0x0FFF

	// Decode instruction
	switch nibble {
	case 0x0:
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
		d.updatePixelBuffer(x, y, n)
	}

	return nil
}

// CLS - 00E0
// Clear the screen of the display (set all the pixels to 'off')
func (d *Device) clearScreen() {
	for i := 0; i < DISPLAY_HEIGHT; i++ {
		for j := 0; j < DISPLAY_WIDTH; j++ {
			d.display[i][j] = false
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
// Draw the display buffer
// TODO
func (d *Device) updatePixelBuffer(x, y, n uint8) {
	vx := d.cpu.generalRegisters[x]
	vy := d.cpu.generalRegisters[y]

	xcoord := vx % DISPLAY_WIDTH
	ycoord := vy % DISPLAY_WIDTH

	d.cpu.generalRegisters[0xF] = 0
}
