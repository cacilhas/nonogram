package ui

import (
	"time"

	raylib "github.com/gen2brain/raylib-go/raylib"
	"github.com/spf13/viper"
)

func Mainloop() {
	scene := NewMenu()

	// TODO: disable ESC key
	for !raylib.WindowShouldClose() {
		if raylib.IsWindowResized() {
			viper.Set("width", raylib.GetScreenWidth())
			viper.Set("height", raylib.GetScreenHeight())
		}

		raylib.BeginDrawing()
		raylib.ClearBackground(raylib.RayWhite)

		scene = scene.Render()

		raylib.EndDrawing()
		time.Sleep(time.Millisecond * 42)
	}
}
