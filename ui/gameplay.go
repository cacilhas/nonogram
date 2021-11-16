package ui

import (
	"fmt"

	"github.com/cacilhas/nonogram/nonogram"
	raygui "github.com/gen2brain/raylib-go/raygui"
	raylib "github.com/gen2brain/raylib-go/raylib"
	"github.com/spf13/viper"
)

type gameplay struct {
	game nonogram.Game
}

func NewGameplay() Scene {
	raylib.SetExitKey(0)
	size := viper.GetInt("size")
	checked := 2. / 3.
	revealed := 0.0
	if viper.GetBool("easy") {
		revealed = 1. / 3.
	}
	return &gameplay{
		game: nonogram.NewGame(size, checked, revealed),
	}
}

func (gp *gameplay) Render() Scene {
	if raylib.IsKeyPressed(raylib.KeyEscape) {
		return NewMenu()
	}

	round := gp.game.Round()
	reference := gp.game.Reference()
	size := round.Size()
	cellSize := 750 / size

	drawColumns(reference, size, cellSize)
	drawLines(reference, size, cellSize)
	drawGrid(round, size, cellSize)

	if gp.game.IsDone() {
		raylib.DrawText("V", 325, 75, 1000, raylib.Green)
	} else {
		checkClick(gp.game, cellSize)
	}

	return gp
}

func drawGrid(round nonogram.Board, size, cellSize int) {
	black := raylib.NewColor(0, 0, 0, 255)
	raygui.SetStyleProperty(raygui.GlobalTextFontsize, int64(cellSize/2))
	for y := 0; y < size; y++ {
		for x := 0; x < size; x++ {
			rect := raylib.Rectangle{
				X:      float32(x*cellSize + 300),
				Y:      float32(y*cellSize + 150),
				Width:  float32(cellSize),
				Height: float32(cellSize),
			}
			var text string
			color := raylib.White

			if ((x/5)+(y/5))%2 == 1 {
				color = raylib.LightGray
			}

			switch round.Get(x, y) {
			case nonogram.CellSet:
				text = ""
				color = raylib.DarkGray
			case nonogram.CellUnset:
				text = "X"
			default:
				text = ""
			}

			raygui.LabelEx(
				rect,
				text,
				black,
				black,
				color,
			)
		}
	}
}

func drawColumns(reference nonogram.Board, size, cellSize int) {
	raygui.SetStyleProperty(raygui.GlobalTextFontsize, 14)
	for x := 0; x < size; x++ {
		current := reference.Column(x)
		for y, value := range current {
			rect := raylib.Rectangle{
				X:      float32(x*cellSize + 308),
				Y:      float32(y * 18),
				Width:  float32(cellSize),
				Height: float32(cellSize),
			}
			raygui.Label(rect, fmt.Sprintf("%d", value))
		}
	}
}

func drawLines(reference nonogram.Board, size, cellSize int) {
	raygui.SetStyleProperty(raygui.GlobalTextFontsize, 14)
	for y := 0; y < size; y++ {
		current := reference.Line(y)
		for x, value := range current {
			rect := raylib.Rectangle{
				X:     float32(x * 18),
				Y:     float32(y*cellSize + 158),
				Width: float32(cellSize),
			}
			raygui.Label(rect, fmt.Sprintf("%d", value))
		}
	}
}

func checkClick(game nonogram.Game, cellSize int) {
	round := game.Round()
	size := round.Size()

	if raylib.IsMouseButtonPressed(raylib.MouseLeftButton) {
		mx := int(raylib.GetMouseX())
		my := int(raylib.GetMouseY())
		x := (mx - 300) / cellSize
		y := (my - 150) / cellSize
		if x >= 0 && x < size && y >= 0 && y < size {
			switch round.Get(x, y) {
			case nonogram.CellSet:
				round.Set(x, y, nonogram.CellUnknown)
			case nonogram.CellUnknown:
				round.Set(x, y, nonogram.CellSet)
				game.Check(x, y)
			default:
			}
		}

	} else if raylib.IsMouseButtonPressed(raylib.MouseRightButton) {
		mx := int(raylib.GetMouseX())
		my := int(raylib.GetMouseY())
		x := (mx - 300) / cellSize
		y := (my - 150) / cellSize
		if x >= 0 && x < size && y >= 0 && y < size {
			switch round.Get(x, y) {
			case nonogram.CellUnset:
				round.Set(x, y, nonogram.CellUnknown)
			case nonogram.CellUnknown:
				round.Set(x, y, nonogram.CellUnset)
			default:
			}
		}
	}
}
