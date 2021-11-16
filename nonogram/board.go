package nonogram

import (
	"fmt"
	"math/rand"
)

type Board interface {
	Size() int
	Get(x, y int) Cell
	Set(x, y int, cell Cell) error
	Column(x int) []uint8
	ColumnStr(x int) string
	Line(x int) []uint8
	LineStr(x int) string
	RevealColumn(x int)
	RevealLine(y int)
	Eq(other Board) bool
}

type board struct {
	size int
	data []Cell
}

func NewBoard(size int) Board {
	return &board{
		size: size,
		data: make([]Cell, size*size),
	}
}

func NewRandomBoard(size int, checked float64) Board {
	b := NewBoard(size)
	for y := 0; y < size; y++ {
		for x := 0; x < size; x++ {
			if rand.Float64() < checked {
				b.Set(x, y, CellSet)
			} else {
				b.Set(x, y, CellUnset)
			}
		}
	}
	return b
}

func (b board) Size() int {
	return b.size
}

func (b board) Get(x, y int) Cell {
	if x < 0 || x >= b.size || y < 0 || y >= b.size {
		return CellUnknown
	}
	return Cell(b.data[y*b.size+x])
}

func (b *board) Set(x, y int, cell Cell) error {
	if x < 0 || x >= b.size || y < 0 || y >= b.size {
		return fmt.Errorf("out of bounds")
	}
	b.data[y*b.size+x] = cell
	return nil
}

func (b board) Column(x int) []uint8 {
	column := make([]uint8, 0)
	counter := uint8(0)
	for y := 0; y < b.size; y++ {
		if b.Get(x, y).IsSet() {
			counter++
		} else {
			if counter > uint8(0) {
				column = append(column, counter)
				counter = uint8(0)
			}
		}
	}
	if counter > 0 || len(column) == 0 {
		column = append(column, counter)
	}
	return column
}

func (b board) ColumnStr(x int) string {
	column := b.Column(x)
	res := make([]byte, len(column))
	for i, c := range column {
		res[i] = c + 0x30
	}
	return string(res)
}

func (b board) Line(y int) []uint8 {
	line := make([]uint8, 0)
	counter := uint8(0)
	for x := 0; x < b.size; x++ {
		if b.Get(x, y).IsSet() {
			counter++
		} else {
			if counter > uint8(0) {
				line = append(line, counter)
				counter = uint8(0)
			}
		}
	}
	if counter > 0 || len(line) == 0 {
		line = append(line, counter)
	}
	return line
}

func (b board) LineStr(y int) string {
	line := b.Line(y)
	res := make([]byte, len(line))
	for i, c := range line {
		res[i] = c + 0x30
	}
	return string(res)
}

func (b *board) RevealColumn(x int) {
	for y := 0; y < b.size; y++ {
		if b.Get(x, y).IsUnknown() {
			b.Set(x, y, CellUnset)
		}
	}
}

func (b *board) RevealLine(y int) {
	for x := 0; x < b.size; x++ {
		if b.Get(x, y).IsUnknown() {
			b.Set(x, y, CellUnset)
		}
	}
}

func (b board) Eq(other Board) bool {
	if b.size != other.Size() {
		return false
	}
	for x := 0; x < b.size; x++ {
		if b.ColumnStr(x) != other.ColumnStr(x) {
			return false
		}
	}
	for y := 0; y < b.size; y++ {
		if b.LineStr(y) != other.LineStr(y) {
			return false
		}
	}
	return true
}
