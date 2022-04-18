package ui

import (
	"fmt"
	"image/color"
	"strings"
	"time"

	"github.com/cacilhas/nonogram/nonogram"
	"github.com/cacilhas/rayframe"
	raygui "github.com/gen2brain/raylib-go/raygui"
	raylib "github.com/gen2brain/raylib-go/raylib"
	"github.com/spf13/viper"
)

type gameplay struct {
	*rayframe.RayFrame
	nonogram.Game
}

type offsetT struct {
	x, y int32
}

func NewGameplay() rayframe.Scene {
	size := viper.GetInt32("size")
	checked := 2. / 3.
	revealed := 0.0
	if viper.GetBool("easy") {
		revealed = 1. / 3.
	}
	return &gameplay{
		Game: nonogram.NewGame(size, checked, revealed),
	}
}

func (gp *gameplay) Background() color.RGBA {
	return raylib.RayWhite
}

func (gp *gameplay) Init(frame *rayframe.RayFrame) {
	gp.RayFrame = frame
	raylib.SetExitKey(0)
}

func (gp *gameplay) Update(dt time.Duration) rayframe.Scene {
	update(dt)
	if raylib.IsKeyPressed(raylib.KeyEscape) {
		return NewMenu()
	}
	if raylib.IsKeyPressed(raylib.KeyF1) {
		return NewHelpPage(gp)
	}
	return gp
}

func (gp *gameplay) Render2D() rayframe.Scene {
	width := int32(gp.WindowSize.X)
	height := int32(gp.WindowSize.Y)
	smaller := height
	if width < smaller {
		smaller = width
	}
	boardSize := int32(float32(smaller) / 1.2)
	if boardSize > 900 {
		boardSize = 900
	}
	offset := offsetT{
		x: width - boardSize,
		y: height - boardSize,
	}
	round := gp.Round()
	reference := gp.Reference()
	size := round.Size()
	cellSize := boardSize / size

	drawLines(reference, size, cellSize, offset)
	drawGrid(round, size, cellSize, offset)
	drawColumns(reference, size, cellSize, offset)

	if !gp.IsDone() {
		checkClick(gp.Game, cellSize, offset)
	}

	return gp
}

func (gp *gameplay) Render3D() rayframe.Scene {
	if gp.IsDone() {
		renderVictory(gp.Camera)
	}
	return gp
}

func drawGrid(round nonogram.Board, size, cellSize int32, offset offsetT) {
	raygui.SetStyleProperty(raygui.GlobalTextFontsize, int64(cellSize/2))
	for y := int32(0); y < size; y++ {
		for x := int32(0); x < size; x++ {
			rect_x := x*cellSize + offset.x
			rect_y := y*cellSize + offset.y
			size := cellSize
			raylib.DrawRectangle(rect_x, rect_y, size, size, raylib.Black)

			cell := round.Get(x, y)
			rest := uint8(255)
			if cell.IsUnknown() {
				rest = uint8(round.Percent(nonogram.CellUnknown)*64 + 192)
			}
			color := raylib.Color{R: 255, G: rest, B: rest, A: 255}
			if ((x/5)+(y/5))%2 == 1 {
				rest = uint8(uint16(rest) * 200 / 256)
				color = raylib.Color{R: 200, G: rest, B: rest, A: 255}
			}

			if cell.IsSet() {
				color = raylib.DarkGray
			}
			raylib.DrawRectangle(rect_x+2, rect_y+2, size-4, size-4, color)
			if cell.IsUnset() {
				raylib.DrawLine(rect_x, rect_y, rect_x+size, rect_y+size, raylib.Black)
				raylib.DrawLine(rect_x+1, rect_y, rect_x+size, rect_y+size-1, raylib.Black)
				raylib.DrawLine(rect_x, rect_y+1, rect_x+size-1, rect_y+size, raylib.Black)

				raylib.DrawLine(rect_x, rect_y+size, rect_x+size, rect_y, raylib.Black)
				raylib.DrawLine(rect_x+1, rect_y+size, rect_x+size, rect_y+1, raylib.Black)
				raylib.DrawLine(rect_x, rect_y+size-1, rect_x+size-1, rect_y, raylib.Black)
			}
		}
	}
	for y := int32(0); y <= size; y += 5 {
		raylib.DrawLine(offset.x, y*cellSize+offset.y, offset.x+cellSize*size, y*cellSize+offset.y, raylib.RayWhite)
	}
	for x := int32(0); x <= size; x += 5 {
		raylib.DrawLine(x*cellSize+offset.x, offset.y, x*cellSize+offset.x, offset.y+cellSize*size, raylib.RayWhite)
	}
}

func drawColumns(reference nonogram.Board, size, cellSize int32, offset offsetT) {
	large := cellSize / 3
	if large < 10 {
		large = 10
	}
	raygui.SetStyleProperty(raygui.GlobalTextFontsize, int64(large-2))
	for x := int32(0); x < size; x++ {
		current := reference.Column(x)
		for y, value := range current {
			rect := raylib.Rectangle{
				X:      float32(x*cellSize + offset.x),
				Y:      float32(int32(y) * large),
				Width:  float32(cellSize),
				Height: float32(offset.y),
			}
			leftRect := raylib.Rectangle{
				X:      rect.X - 1,
				Y:      rect.Y - 1,
				Width:  rect.Width,
				Height: rect.Height,
			}
			rightRect := raylib.Rectangle{
				X:      rect.X + 1,
				Y:      rect.Y + 1,
				Width:  rect.Width,
				Height: rect.Height,
			}
			raygui.LabelEx(leftRect, fmt.Sprintf("%d", value), raylib.White, raylib.Color{255, 255, 255, 0}, raylib.Color{255, 255, 255, 0})
			raygui.LabelEx(rightRect, fmt.Sprintf("%d", value), raylib.White, raylib.Color{255, 255, 255, 0}, raylib.Color{255, 255, 255, 0})
			raygui.LabelEx(rect, fmt.Sprintf("%d", value), raylib.DarkGray, raylib.Color{255, 255, 255, 0}, raylib.Color{255, 255, 255, 0})
		}
	}
}

func drawLines(reference nonogram.Board, size, cellSize int32, offset offsetT) {
	large := cellSize / 3
	if large < 10 {
		large = 10
	}
	width := float32(200)
	if width > float32(offset.x) {
		width = float32(offset.x)
	}
	x := float32(offset.x) - width
	raygui.SetStyleProperty(raygui.GlobalTextFontsize, int64(large-2))
	for y := int32(0); y < size; y++ {
		current := reference.Line(y)
		line := ""
		for _, value := range current {
			line = fmt.Sprintf("%s  %d", line, value)
		}
		rect := raylib.Rectangle{
			X:      x,
			Y:      float32(y*cellSize + offset.y),
			Width:  width,
			Height: float32(cellSize),
		}
		raygui.Label(rect, strings.TrimSpace(line))
	}
}

func checkClick(game nonogram.Game, cellSize int32, offset offsetT) {
	round := game.Round()
	size := round.Size()
	mx := raylib.GetMouseX()
	my := raylib.GetMouseY()
	x := (mx - offset.x) / cellSize
	y := (my - offset.y) / cellSize

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
