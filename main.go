package main

import (
	"log"
	"os"

	"github.com/hajimehoshi/ebiten"
)

func main() {
	// Setup
	device := newDevice()

	ebiten.SetWindowSize(DISPLAY_WIDTH*2, DISPLAY_HEIGHT*2)
	ebiten.SetWindowTitle("Game of Life (Ebiten Demo)")

	// Check if a rom was provided
	if len(os.Args) < 2 {
		log.Println("Not enough arguments provided.\nUsage: 'chip-8 <path-to-rom>'")
		return
	}

	// Load ROM into RAM and load font
	args := os.Args[1:]
	romFilePath := args[0]
	rom, err := os.Open(romFilePath)
	if err != nil {
		panic(err)
	}
	defer rom.Close()

	device.LoadROM(rom)
	device.LoadFont()

	// Run emulation cycle here
	if err := ebiten.RunGame(device); err != nil {
		log.Fatal(err)
	}
}
