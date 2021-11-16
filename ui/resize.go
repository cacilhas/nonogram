package ui

import (
	raygui "github.com/gen2brain/raylib-go/raygui"
	raylib "github.com/gen2brain/raylib-go/raylib"
	"github.com/spf13/viper"
)

func renderResizer(width, height int32) {
	rect := raylib.Rectangle{}
	raygui.ConstrainRectangle(&rect, 10, 32, 10, 32)
	rect.X = float32(width) - rect.Width - 2
	rect.Y = 2
	viper.Set("fullscreen", raygui.CheckBox(rect, raylib.IsWindowFullscreen()))
}
