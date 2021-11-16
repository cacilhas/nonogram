package ui

import (
	raygui "github.com/gen2brain/raylib-go/raygui"
	raylib "github.com/gen2brain/raylib-go/raylib"
	"github.com/spf13/viper"
)

type mainMenu struct {
}

func NewMenu() Scene {
	return &mainMenu{}
}

func (m *mainMenu) Init() Scene {
	raylib.SetExitKey(raylib.KeyEscape)
	return m
}

func (m *mainMenu) Render() Scene {
	if raylib.IsKeyPressed(raylib.KeyF1) {
		return NewHelpPage(m).Init()
	}

	raygui.SetStyleProperty(raygui.GlobalTextFontsize, 120)
	raygui.Label(
		raylib.Rectangle{X: 214, Y: 30, Width: 772, Height: 114},
		"Nonogram",
	)

	raygui.SetStyleProperty(raygui.GlobalTextFontsize, 72)

	if raygui.CheckBox(
		raylib.Rectangle{X: 360, Y: 234, Width: 80, Height: 60},
		viper.GetInt("size") == 5,
	) {
		viper.Set("size", 5)
	}
	raygui.Label(
		raylib.Rectangle{X: 448, Y: 225, Width: 400, Height: 60},
		"5 x 5",
	)

	if raygui.CheckBox(
		raylib.Rectangle{X: 360, Y: 354, Width: 80, Height: 60},
		viper.GetInt("size") == 10,
	) {
		viper.Set("size", 10)
	}
	raygui.Label(
		raylib.Rectangle{X: 448, Y: 345, Width: 400, Height: 60},
		"10 x 10",
	)

	if raygui.CheckBox(
		raylib.Rectangle{X: 360, Y: 474, Width: 80, Height: 60},
		viper.GetInt("size") == 15,
	) {
		viper.Set("size", 15)
	}
	raygui.Label(
		raylib.Rectangle{X: 448, Y: 465, Width: 400, Height: 60},
		"15 x 15",
	)

	viper.Set(
		"easy",
		raygui.CheckBox(
			raylib.Rectangle{X: 360, Y: 624, Width: 80, Height: 60},
			viper.GetBool("easy"),
		),
	)
	raygui.Label(
		raylib.Rectangle{X: 448, Y: 615, Width: 400, Height: 60},
		"Easy",
	)

	raygui.SetStyleProperty(raygui.GlobalTextFontsize, 84)
	if raygui.Button(
		raylib.Rectangle{X: 400, Y: 774, Width: 400, Height: 100},
		"Play",
	) || raylib.IsKeyPressed(raylib.KeyEnter) || raylib.IsKeyPressed(raylib.KeyKpEnter) {
		return NewGameplay().Init()
	}

	raygui.SetStyleProperty(raygui.GlobalTextFontsize, 10)
	raygui.Label(
		raylib.Rectangle{X: 1100, Y: 880, Width: 72, Height: 10},
		"F1 for help",
	)

	return m
}
