package main

const NUM_REGISTERS = 16

type CPU struct {
	generalRegisters [NUM_REGISTERS]uint8
	delayTimer       uint8
	soundTimer       uint8
	programCounter   uint16
	stackPointer     uint16
	indexRegister    uint16
}
