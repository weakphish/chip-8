package main

const numRegisters = 16

type CPU struct {
	generalRegisters [numRegisters]uint8
	delayTimer       uint8
	soundTimer       uint8
	programCounter   uint16
	stackPointer     uint16
	indexRegister    uint16
}
