package nonogram

import "math/rand"

type Game interface {
	Check(x, y int32)
	IsDone() bool
	Reference() Board
	Round() Board
}

type game struct {
	reference, round Board
}

func NewGame(size int32, checked, revealed float64) *game {
	res := &game{
		reference: NewRandomBoard(size, checked),
		round:     NewBoard(size),
	}
	if revealed != 0.0 {
		for y := int32(0); y < size; y++ {
			for x := int32(0); x < size; x++ {
				if res.reference.Get(x, y).IsUnset() && rand.Float64() < revealed {
					res.round.Set(x, y, CellUnset)
				}
			}
		}
	}
	return res
}

func (g *game) Check(x, y int32) {
	if g.reference.ColumnStr(x) == g.round.ColumnStr(x) {
		g.round.RevealColumn(x)
	}
	if g.reference.LineStr(y) == g.round.LineStr(y) {
		g.round.RevealLine(y)
	}
}

func (g game) IsDone() bool {
	return g.reference.Eq(g.round)
}

func (g game) Reference() Board {
	return g.reference
}

func (g game) Round() Board {
	return g.round
}
