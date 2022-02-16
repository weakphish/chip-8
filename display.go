package main

const (
	DISPLAY_HEIGHT = 32
	DISPLAY_WIDTH  = 64
)

type Display struct {
	frame [DISPLAY_HEIGHT][DISPLAY_WIDTH]bool
}
