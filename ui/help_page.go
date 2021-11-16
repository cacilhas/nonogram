package ui

import (
	"fmt"
	"os/exec"
	"runtime"

	raygui "github.com/gen2brain/raylib-go/raygui"
	raylib "github.com/gen2brain/raylib-go/raylib"
	"github.com/spf13/viper"
)

var commands = map[string]string{
	"darwin":  "open",
	"linux":   "xdg-open",
	"windows": "cmd /c start",
}

type helpPage struct {
	previous Scene
}

func NewHelpPage(previous Scene) Scene {
	return &helpPage{previous: previous}
}

func (h *helpPage) Init() Scene {
	raylib.SetExitKey(0)
	return h
}

func (hp *helpPage) Render() Scene {
	if raylib.IsKeyPressed(raylib.KeyEscape) {
		return hp.previous.Init()
	}

	raygui.SetStyleProperty(raygui.GlobalTextFontsize, 120)
	raygui.Label(
		raylib.Rectangle{X: 214, Y: 30, Width: 772, Height: 114},
		"Nonogram",
	)

	raygui.SetStyleProperty(raygui.GlobalTextFontsize, 72)
	raygui.LabelEx(
		raylib.Rectangle{X: 300, Y: 225, Width: 600, Height: 60},
		fmt.Sprintf("Version %s", viper.GetString("version")),
		raylib.White,
		raylib.LightGray,
		raylib.LightGray,
	)

	copying := "Copyright ©2021 Arhimedes Montegasppa Cacilhas <batalema@cacilhas.info>"
	raygui.SetStyleProperty(raygui.GlobalTextFontsize, 14)
	if raygui.Button(
		raylib.Rectangle{X: 300, Y: 359, Width: 600, Height: 14},
		copying,
	) {
		openURL("https://opensource.org/licenses/BSD-3-Clause")
	}

	homepage := viper.GetString("homepage")
	if raygui.Button(
		raylib.Rectangle{X: 300, Y: 387, Width: 600, Height: 14},
		fmt.Sprintf("Homepage: %s", homepage),
	) {
		openURL(homepage)
	}

	raygui.SetStyleProperty(raygui.GlobalTextFontsize, 32)
	raylib.DrawRectangle(300, 428, 600, 32, raylib.LightGray)
	raygui.LabelEx(
		raylib.Rectangle{X: 300, Y: 460, Width: 600, Height: 40},
		"Left click to set or unset a cell",
		raylib.White,
		raylib.LightGray,
		raylib.LightGray,
	)
	raygui.LabelEx(
		raylib.Rectangle{X: 300, Y: 500, Width: 600, Height: 40},
		"Right click to block or unblock a cell",
		raylib.White,
		raylib.LightGray,
		raylib.LightGray,
	)
	raygui.LabelEx(
		raylib.Rectangle{X: 300, Y: 540, Width: 600, Height: 40},
		"Esc to quit",
		raylib.White,
		raylib.LightGray,
		raylib.LightGray,
	)
	raygui.LabelEx(
		raylib.Rectangle{X: 300, Y: 580, Width: 600, Height: 40},
		"F1 for help",
		raylib.White,
		raylib.LightGray,
		raylib.LightGray,
	)
	raylib.DrawRectangle(300, 620, 600, 32, raylib.LightGray)

	return hp
}

func openURL(uri string) {
	run, _ := commands[runtime.GOOS]
	exec.Command(run, uri).Start()
}
