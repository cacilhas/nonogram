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
		if viper.GetBool("fullscreen") && !raylib.IsWindowFullscreen() {
			raylib.ToggleFullscreen()
			raylib.SetWindowSize(viper.GetInt("width"), viper.GetInt("height"))
		} else if !viper.GetBool("fullscreen") && raylib.IsWindowFullscreen() {
			raylib.ToggleFullscreen()
			width, height := GetSysResolution()
			raylib.SetWindowSize(int(width), int(height))
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
		return GetSysResolution()
	}
	return viper.GetInt32("width"), viper.GetInt32("height")
}
