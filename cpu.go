package main

const NUM_REGISTERS = 16

// In our implementation, the CPU doesn't actually perform any computation - it just is a holder
// for the various registers used by the Chip 8.
type CPU struct {
	generalRegisters [NUM_REGISTERS]uint8
	delayTimer       uint8
	soundTimer       uint8
	programCounter   uint16
	stackPointer     uint16
	indexRegister    uint16
}

func newCPU() *CPU {
	return &CPU{programCounter: PROG_MEM_START}
}
