package ui

import (
	"time"

	raylib "github.com/gen2brain/raylib-go/raylib"
)

func Mainloop() {
	scene := NewMenu()

	// TODO: disable ESC key
	for !raylib.WindowShouldClose() {
		raylib.BeginDrawing()
		raylib.ClearBackground(raylib.RayWhite)

		scene = scene.Render()

		raylib.EndDrawing()
		time.Sleep(time.Millisecond * 42)
	}
}
