package ui

import (
	"image/color"
	"time"

	"github.com/cacilhas/rayframe"
	raygui "github.com/gen2brain/raylib-go/raygui"
	raylib "github.com/gen2brain/raylib-go/raylib"
	"github.com/spf13/viper"
)

type mainMenu struct {
	*rayframe.RayFrame
}

func NewMenu() rayframe.Scene {
	return &mainMenu{}
}

func (menu *mainMenu) Init(frame *rayframe.RayFrame) {
	menu.RayFrame = frame
	raylib.SetExitKey(raylib.KeyEscape)
}

func (menu *mainMenu) Background() color.RGBA {
	return raylib.RayWhite
}

func (menu *mainMenu) Update(dt time.Duration) rayframe.Scene {
	update(dt)
	if raylib.IsKeyPressed(raylib.KeyF1) {
		return NewHelpPage(menu)
	}
	return menu
}

func (menu *mainMenu) Render2D() rayframe.Scene {
	width := menu.WindowSize.X
	height := menu.WindowSize.Y
	// TODO: fix fullscreen
	renderResizer(int32(width), int32(height))
	bigFontSize := int64(float32(height) / 7.5)
	if bigFontSize > 120 {
		bigFontSize = 120
	}
	menuFontSize := int64(float32(height) / 12.5)
	helpFontSize := int64(float32(height) / 80)
	if helpFontSize < 10 {
		helpFontSize = 10
	}
	boxSize := float32(height) / 15
	buttonFontSize := int64(float32(height) / 10)
	boxX := float32(width) / 3.5
	boxY := float32(height) / 3.85
	textSize := float32(menuFontSize) * 5
	textX := boxX + float32(menuFontSize)*1.22

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

	raygui.SetStyleProperty(raygui.GlobalTextFontsize, menuFontSize)

	if raygui.CheckBox(
		raylib.Rectangle{
			X:      boxX,
			Y:      boxY,
			Width:  boxSize,
			Height: boxSize,
		},
		viper.GetInt32("size") == 5,
	) {
		viper.Set("size", 5)
	}
	textY := boxY + boxSize/2 - float32(menuFontSize)/2
	raygui.Label(
		raylib.Rectangle{
			X:      textX,
			Y:      textY,
			Width:  textSize,
			Height: float32(menuFontSize),
		},
		"5 x 5",
	)

	boxY += float32(bigFontSize)
	if raygui.CheckBox(
		raylib.Rectangle{
			X:      boxX,
			Y:      boxY,
			Width:  boxSize,
			Height: boxSize,
		},
		viper.GetInt32("size") == 10,
	) {
		viper.Set("size", 10)
	}
	textY = boxY + boxSize/2 - float32(menuFontSize)/2
	raygui.Label(
		raylib.Rectangle{
			X:      textX,
			Y:      textY,
			Width:  textSize,
			Height: float32(menuFontSize),
		},
		"10 x 10",
	)

	boxY += float32(bigFontSize)
	if raygui.CheckBox(
		raylib.Rectangle{
			X:      boxX,
			Y:      boxY,
			Width:  boxSize,
			Height: boxSize,
		},
		viper.GetInt32("size") == 15,
	) {
		viper.Set("size", 15)
	}
	textY = boxY + boxSize/2 - float32(menuFontSize)/2
	raygui.Label(
		raylib.Rectangle{
			X:      textX,
			Y:      textY,
			Width:  textSize,
			Height: float32(menuFontSize),
		},
		"15 x 15",
	)

	boxY += float32(bigFontSize) * 1.25
	viper.Set(
		"easy",
		raygui.CheckBox(
			raylib.Rectangle{
				X:      boxX,
				Y:      boxY,
				Width:  boxSize,
				Height: boxSize,
			},
			viper.GetBool("easy"),
		),
	)
	textY = boxY + boxSize/2 - float32(menuFontSize)/2
	raygui.Label(
		raylib.Rectangle{
			X:      textX,
			Y:      textY,
			Width:  textSize,
			Height: float32(menuFontSize),
		},
		"Easy",
	)

	boxY += float32(bigFontSize) * 1.25
	textSize = float32(buttonFontSize) * 3
	raygui.SetStyleProperty(raygui.GlobalTextFontsize, buttonFontSize)
	if raygui.Button(
		raylib.Rectangle{
			X:      (float32(width) - textSize) / 2,
			Y:      boxY,
			Width:  textSize,
			Height: float32(buttonFontSize) * 1.2,
		},
		"Play",
	) || raylib.IsKeyPressed(raylib.KeyEnter) || raylib.IsKeyPressed(raylib.KeyKpEnter) {
		return NewGameplay()
	}

	raygui.SetStyleProperty(raygui.GlobalTextFontsize, helpFontSize)
	textSize = float32(helpFontSize) * 6
	raygui.Label(
		raylib.Rectangle{
			X:      (float32(width) - textSize) - float32(helpFontSize),
			Y:      float32(height) - float32(helpFontSize)*2,
			Width:  textSize,
			Height: float32(helpFontSize),
		},
		"F1 for help",
	)

	return menu
}
