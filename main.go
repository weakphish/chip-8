package main

import (
	"github.com/faiface/pixel"
	"github.com/faiface/pixel/pixelgl"
	"golang.org/x/image/colornames"
)

// Locks the program to a single thread. In essence, replaces our 'main' function. This is because
// PixelGL needs to use the main thread for graphics, while we can use other threads as we want.
func run() {
	cfg := pixelgl.WindowConfig{
		Title:  "Pixel Rocks!",
		Bounds: pixel.R(0, 0, 1024, 768),
		VSync:  true,
	}
	win, err := pixelgl.NewWindow(cfg)
	if err != nil {
		panic(err)
	}

	win.Clear(colornames.Skyblue)

	for !win.Closed() {
		win.Update()
	}
}

func main() {
	pixelgl.Run(run)
}
