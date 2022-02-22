package main

import (
	"os"

	"github.com/faiface/pixel"
	"github.com/faiface/pixel/pixelgl"
	"golang.org/x/image/colornames"
)

// Locks the program to a single thread. In essence, replaces our 'main' function. This is because
// PixelGL needs to use the main thread for graphics, while we can use other threads as we want.
func run() {
	cfg := pixelgl.WindowConfig{
		Title:  "Chip-8",
		Bounds: pixel.R(0, 0, DISPLAY_WIDTH, DISPLAY_HEIGHT),
		VSync:  true,
	}
	win, err := pixelgl.NewWindow(cfg)
	if err != nil {
		panic(err)
	}

	win.Clear(colornames.Skyblue)

	device := newDevice()

	// Load ROM into RAM and load font
	args := os.Args[1:]
	romFilePath := args[0]
	rom, err := os.Open(romFilePath)
	if err != nil {
		panic(err)
	}
	defer rom.Close()

	device.ram.LoadROM(rom)
	device.ram.LoadFont()

	// Run emulation cycle here
	for !win.Closed() {
		device.emulateCycle()
		// TODO update screen here
		win.Update()
	}
}

func main() {
	pixelgl.Run(run)
}
