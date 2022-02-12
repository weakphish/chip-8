package main

const STACK_SIZE = 16

type Device struct {
	ram   Memory
	cpu   CPU
	stack [STACK_SIZE]uint16
}
