package ui

import (
	"time"

	raylib "github.com/gen2brain/raylib-go/raylib"
	"github.com/spf13/viper"
)

var fsWidth, fsHeight int32

func Mainloop() {
	scene := NewMenu()

	// TODO: disable ESC key
	for !raylib.WindowShouldClose() {
		if viper.GetBool("fullscreen") && !raylib.IsWindowFullscreen() {
			raylib.ToggleFullscreen()
		} else if !viper.GetBool("fullscreen") && raylib.IsWindowFullscreen() {
			raylib.ToggleFullscreen()
		}

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

func getSize() (int32, int32) {
	if raylib.IsWindowFullscreen() {
		if fsWidth == 0 || fsHeight == 0 {
			fsWidth, fsHeight = GetResolution()
		}
		return fsWidth, fsHeight
	}
	return viper.GetInt32("width"), viper.GetInt32("height")
}
