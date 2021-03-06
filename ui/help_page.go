package ui

import (
	"fmt"
	"image/color"
	"os/exec"
	"runtime"
	"time"

	"github.com/cacilhas/rayframe"
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
	*rayframe.RayFrame
	previous rayframe.Scene
}

func NewHelpPage(previous rayframe.Scene) rayframe.Scene {
	return &helpPage{previous: previous}
}

func (help *helpPage) Init(frame *rayframe.RayFrame) {
	help.RayFrame = frame
}

func (help helpPage) ExitKey() int32 {
	return 0
}

func (help helpPage) OnKeyEscape() rayframe.Scene {
	return help.previous
}

func (help helpPage) Background() color.RGBA {
	return raylib.RayWhite
}

func (help helpPage) Update(dt time.Duration) rayframe.Scene {
	update(dt)
	return help
}

func (help helpPage) Render2D() rayframe.Scene {
	width := help.WindowSize.X
	height := help.WindowSize.Y
	widthThird := float32(width) / 3
	bigFontSize := int64(float32(height) / 7.5)
	if bigFontSize > 120 {
		bigFontSize = 120
	}
	subtitleFontSize := int64(float32(height) / 12.5)
	smallFontSize := bigFontSize / 8
	if smallFontSize < 10 {
		smallFontSize = 10
	}
	fontMiddleSize := int64(float32(bigFontSize) / 3.75)
	if fontMiddleSize < 24 {
		fontMiddleSize = 24
	}

	titleWidth := float32(bigFontSize) * 6.5
	raygui.SetStyleProperty(raygui.GlobalTextFontsize, bigFontSize)
	raygui.Label(
		raylib.Rectangle{
			X:      (float32(width) - titleWidth) / 2,
			Y:      float32(height) / 30,
			Width:  titleWidth,
			Height: float32(bigFontSize),
		},
		"Nonogram",
	)

	y := float32(height / 4)
	raygui.SetStyleProperty(raygui.GlobalTextFontsize, subtitleFontSize)
	raygui.LabelEx(
		raylib.Rectangle{
			X:      widthThird / 2,
			Y:      y,
			Width:  widthThird * 2,
			Height: float32(subtitleFontSize),
		},
		fmt.Sprintf("Version %s", viper.GetString("version")),
		raylib.White,
		raylib.LightGray,
		raylib.LightGray,
	)

	y += float32(bigFontSize)
	copying := "Copyright ??2021 Arhimedes Montegasppa Cacilhas <batalema@cacilhas.info>"
	raygui.SetStyleProperty(raygui.GlobalTextFontsize, smallFontSize)
	if raygui.Button(
		raylib.Rectangle{
			X:      widthThird / 2,
			Y:      y,
			Width:  widthThird * 2,
			Height: float32(fontMiddleSize),
		},
		copying,
	) {
		openURL("https://opensource.org/licenses/BSD-3-Clause")
	}

	y += float32(fontMiddleSize)
	homepage := viper.GetString("homepage")
	if raygui.Button(
		raylib.Rectangle{
			X:      widthThird / 2,
			Y:      y,
			Width:  widthThird * 2,
			Height: float32(fontMiddleSize),
		},
		fmt.Sprintf("Homepage: %s", homepage),
	) {
		openURL(homepage)
	}

	blockSize := float32(fontMiddleSize) * 1.25
	y += float32(fontMiddleSize)
	raygui.SetStyleProperty(raygui.GlobalTextFontsize, fontMiddleSize)
	raylib.DrawRectangle(
		int32(widthThird/2), int32(y),
		int32(widthThird*2), int32(fontMiddleSize),
		raylib.LightGray,
	)
	y += float32(fontMiddleSize)
	raygui.LabelEx(
		raylib.Rectangle{
			X:      widthThird / 2,
			Y:      y,
			Width:  widthThird * 2,
			Height: blockSize,
		},
		"Left click to set or unset a cell",
		raylib.White,
		raylib.LightGray,
		raylib.LightGray,
	)
	y += blockSize
	raygui.LabelEx(
		raylib.Rectangle{
			X:      widthThird / 2,
			Y:      y,
			Width:  widthThird * 2,
			Height: blockSize,
		},
		"Right click to block or unblock a cell",
		raylib.White,
		raylib.LightGray,
		raylib.LightGray,
	)
	y += blockSize
	raygui.LabelEx(
		raylib.Rectangle{
			X:      widthThird / 2,
			Y:      y,
			Width:  widthThird * 2,
			Height: blockSize,
		},
		"Esc to quit",
		raylib.White,
		raylib.LightGray,
		raylib.LightGray,
	)
	y += blockSize
	raygui.LabelEx(
		raylib.Rectangle{
			X:      widthThird / 2,
			Y:      y,
			Width:  widthThird * 2,
			Height: blockSize,
		},
		"F1 for help",
		raylib.White,
		raylib.LightGray,
		raylib.LightGray,
	)
	y += blockSize
	raylib.DrawRectangle(
		int32(widthThird/2), int32(y),
		int32(widthThird*2), int32(fontMiddleSize),
		raylib.LightGray,
	)

	return help
}

func openURL(uri string) {
	run, _ := commands[runtime.GOOS]
	exec.Command(run, uri).Start()
}
