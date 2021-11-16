package main

import (
	"math/rand"
	"time"

	"github.com/cacilhas/nonogram/ui"
	raylib "github.com/gen2brain/raylib-go/raylib"
)

func init() {
	rand.Seed(time.Now().UnixNano())
}

func main() {
	readSettings()
	defer saveSettings()

	raylib.InitWindow(1200, 900, "Nonogram")
	raylib.SetTargetFPS(24)

	ui.Mainloop()

	raylib.CloseWindow()
}
