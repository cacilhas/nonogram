package main

import (
	"math/rand"
	"time"

	"github.com/cacilhas/nonogram/ui"
	"github.com/cacilhas/rayframe"
	raylib "github.com/gen2brain/raylib-go/raylib"
	"github.com/spf13/viper"
)

func init() {
	rand.Seed(time.Now().UnixNano())
}

func main() {
	readSettings()
	defer saveSettings()

	camera := raylib.Camera{
		Position:   raylib.Vector3{X: 10.0, Y: 10.0, Z: 8.0},
		Target:     raylib.Vector3{},
		Up:         raylib.Vector3{X: 0.0, Y: 1.0, Z: 0.0},
		Fovy:       60,
		Projection: raylib.CameraPerspective,
	}
	frame := &rayframe.RayFrame{
		Camera: &camera,
		FPS:    24,
	}
	frame.Init(
		viper.GetInt("width"),
		viper.GetInt("height"),
		"Nonogram",
	)
	frame.OnResize = func(width, height int, fullscreen bool) {
		if !fullscreen {
			viper.Set("width", width)
			viper.Set("height", height)
		}
	}
	raylib.SetWindowMinSize(800, 600)
	raylib.SetWindowState(raylib.FlagWindowResizable)
	frame.Mainloop(ui.NewMenu())
}
