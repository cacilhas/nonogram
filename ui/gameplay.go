package ui

import (
	"fmt"
	"strings"

	"github.com/cacilhas/nonogram/nonogram"
	raygui "github.com/gen2brain/raylib-go/raygui"
	raylib "github.com/gen2brain/raylib-go/raylib"
	"github.com/spf13/viper"
)

type gameplay struct {
	game nonogram.Game
}

type offsetT struct {
	x, y int32
}

func NewGameplay() Scene {
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

func (gp *gameplay) Init() Scene {
	raylib.SetExitKey(0)
	return gp
}

func (gp *gameplay) Render() Scene {
	if raylib.IsKeyPressed(raylib.KeyEscape) {
		return NewMenu().Init()
	}
	if raylib.IsKeyPressed(raylib.KeyF1) {
		return NewHelpPage(gp).Init()
	}

	width := viper.GetInt32("width")
	height := viper.GetInt32("height")
	smaller := width
	if height < smaller {
		smaller = height
	}
	boardSize := int32(float32(smaller) / 1.2)
	if boardSize > 750 {
		boardSize = 750
	}
	offset := offsetT{
		x: width - boardSize,
		y: height - boardSize,
	}
	round := gp.game.Round()
	reference := gp.game.Reference()
	size := round.Size()
	cellSize := int(boardSize) / size

	drawColumns(reference, size, cellSize, offset)
	drawLines(reference, size, cellSize, offset)
	drawGrid(round, size, cellSize, offset)

	if gp.game.IsDone() {
		raylib.DrawText("V", offset.x, offset.y, boardSize, raylib.Green)
	} else {
		checkClick(gp.game, cellSize, offset)
	}

	return gp
}

func drawGrid(round nonogram.Board, size, cellSize int, offset offsetT) {
	black := raylib.Color{R: 0, G: 0, B: 0, A: 255}
	raygui.SetStyleProperty(raygui.GlobalTextFontsize, int64(cellSize/2))
	for y := 0; y < size; y++ {
		for x := 0; x < size; x++ {
			rect_x := int32(x*cellSize) + offset.x
			rect_y := int32(y*cellSize) + offset.y
			size := int32(cellSize)
			raylib.DrawRectangle(rect_x, rect_y, size, size, black)

			color := raylib.White
			if ((x/5)+(y/5))%2 == 1 {
				color = raylib.LightGray
			}

			cell := round.Get(x, y)
			if cell.IsSet() {
				color = raylib.DarkGray
			}
			raylib.DrawRectangle(rect_x+2, rect_y+2, size-4, size-4, color)
			if cell.IsUnset() {
				raylib.DrawLine(rect_x, rect_y, rect_x+size, rect_y+size, black)
				raylib.DrawLine(rect_x+1, rect_y, rect_x+size, rect_y+size-1, black)
				raylib.DrawLine(rect_x, rect_y+1, rect_x+size-1, rect_y+size, black)

				raylib.DrawLine(rect_x, rect_y+size, rect_x+size, rect_y, black)
				raylib.DrawLine(rect_x+1, rect_y+size, rect_x+size, rect_y+1, black)
				raylib.DrawLine(rect_x, rect_y+size-1, rect_x+size-1, rect_y, black)
			}
		}
	}
}

func drawColumns(reference nonogram.Board, size, cellSize int, offset offsetT) {
	large := cellSize / 3
	if large < 10 {
		large = 10
	}
	raygui.SetStyleProperty(raygui.GlobalTextFontsize, int64(large-2))
	for x := 0; x < size; x++ {
		current := reference.Column(x)
		for y, value := range current {
			rect := raylib.Rectangle{
				X:      float32(x*cellSize + int(offset.x)),
				Y:      float32(y * large),
				Width:  float32(cellSize),
				Height: float32(offset.y),
			}
			raygui.Label(rect, fmt.Sprintf("%d", value))
		}
	}
}

func drawLines(reference nonogram.Board, size, cellSize int, offset offsetT) {
	large := cellSize / 3
	if large < 10 {
		large = 10
	}
	raygui.SetStyleProperty(raygui.GlobalTextFontsize, int64(large-2))
	for y := 0; y < size; y++ {
		current := reference.Line(y)
		line := ""
		for _, value := range current {
			line = fmt.Sprintf("%s  %d", line, value)
		}
		rect := raylib.Rectangle{
			X:      0,
			Y:      float32(y*cellSize + int(offset.y)),
			Width:  float32(offset.x),
			Height: float32(cellSize),
		}
		raygui.Label(rect, strings.TrimSpace(line))
	}
}

func checkClick(game nonogram.Game, cellSize int, offset offsetT) {
	round := game.Round()
	size := round.Size()
	mx := int(raylib.GetMouseX())
	my := int(raylib.GetMouseY())
	x := (mx - int(offset.x)) / cellSize
	y := (my - int(offset.y)) / cellSize

	if x >= 0 && x < size && y >= 0 && y < size {
		if raylib.IsMouseButtonPressed(raylib.MouseLeftButton) {
			switch round.Get(x, y) {
			case nonogram.CellSet:
				round.Set(x, y, nonogram.CellUnknown)
			case nonogram.CellUnknown:
				round.Set(x, y, nonogram.CellSet)
				game.Check(x, y)
			default:
			}

		} else if raylib.IsMouseButtonPressed(raylib.MouseRightButton) {
			switch round.Get(x, y) {
			case nonogram.CellUnset:
				round.Set(x, y, nonogram.CellUnknown)
			case nonogram.CellUnknown:
				round.Set(x, y, nonogram.CellUnset)
				game.Check(x, y)
			default:
			}
		}
	}
}
