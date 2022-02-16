package main

const STACK_SIZE = 16

type Device struct {
	ram RAM
	cpu CPU
	// the stack is defined in the top level device so that the methods using it can access the stack pointer
	// via the CPU
	stack [STACK_SIZE]uint16
}
