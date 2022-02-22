package main

type Stack struct {
	spPointer *uint16
	stack     [STACK_SIZE]uint16
}

func newStack(sp *uint16) *Stack {
	return &Stack{spPointer: sp}
}

func (s *Stack) push(value uint16) {
	s.stack[*s.spPointer] = value
	*s.spPointer++
}

func (s *Stack) pop() uint16 {
	val := s.stack[*s.spPointer]
	*s.spPointer--
	return val
}
