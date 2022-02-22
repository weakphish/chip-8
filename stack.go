package main

type Stack struct {
	spPointer *uint16
	arr       [STACK_SIZE]uint16
}

func newStack(sp *uint16) *Stack {
	return &Stack{spPointer: sp}
}

func (s *Stack) push(value uint16) {
	// TODO should probably implement indexing protections
	s.arr[*s.spPointer] = value
	*s.spPointer++
}

func (s *Stack) pop() uint16 {
	// TODO should probably implement indexing protections
	val := s.arr[*s.spPointer]
	*s.spPointer--
	return val
}
