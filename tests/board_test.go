package tests

import (
	"math/rand"
	"testing"

	"github.com/cacilhas/nonogram/nonogram"
)

func TestBoard(t *testing.T) {
	t.Run("new empty board", func(t *testing.T) {
		board := nonogram.NewBoard(2)
		if got := board.Size(); got != 2 {
			t.Fatalf("expected 2, got %v", got)
		}
		for y := 0; y < 2; y++ {
			for x := 0; x < 2; x++ {
				if got := board.Get(x, y); !got.IsUnknown() {
					t.Fatalf("expected unknown at [%v, %v], got %v", x, y, got)
				}
			}
		}
	})

	t.Run("new random board", func(t *testing.T) {
		rand.Seed(1)
		board := nonogram.NewRandomBoard(4, 0.5)
		if got := board.Get(0, 0); !got.IsUnset() {
			t.Fatalf("expected unset at [0, 0], got %v", got)
		}
		if got := board.Get(1, 0); !got.IsUnset() {
			t.Fatalf("expected unset at [1, 0], got %v", got)
		}
		if got := board.Get(2, 0); !got.IsUnset() {
			t.Fatalf("expected unset at [2, 0], got %v", got)
		}
		if got := board.Get(3, 0); !got.IsSet() {
			t.Fatalf("expected set at [3, 0], got %v", got)
		}
		if got := board.Get(0, 1); !got.IsSet() {
			t.Fatalf("expected set at [0, 1], got %v", got)
		}
		if got := board.Get(1, 1); !got.IsUnset() {
			t.Fatalf("expected unset at [1, 1], got %v", got)
		}
		if got := board.Get(2, 1); !got.IsSet() {
			t.Fatalf("expected set at [2, 1], got %v", got)
		}
		if got := board.Get(3, 1); !got.IsSet() {
			t.Fatalf("expected set at [3, 1], got %v", got)
		}
		if got := board.Get(0, 2); !got.IsSet() {
			t.Fatalf("expected set at [0, 2], got %v", got)
		}
		if got := board.Get(1, 2); !got.IsSet() {
			t.Fatalf("expected set at [1, 2], got %v", got)
		}
		if got := board.Get(2, 2); !got.IsUnset() {
			t.Fatalf("expected unset at [2, 2], got %v", got)
		}
		if got := board.Get(3, 2); !got.IsUnset() {
			t.Fatalf("expected unset at [3, 2], got %v", got)
		}
		if got := board.Get(0, 3); !got.IsSet() {
			t.Fatalf("expected set at [0, 3], got %v", got)
		}
		if got := board.Get(1, 3); !got.IsSet() {
			t.Fatalf("expected set at [1, 3], got %v", got)
		}
		if got := board.Get(2, 3); !got.IsSet() {
			t.Fatalf("expected set at [2, 3], got %v", got)
		}
		if got := board.Get(3, 3); !got.IsSet() {
			t.Fatalf("expected set at [3, 3], got %v", got)
		}
	})

	t.Run("set cell", func(t *testing.T) {
		board := nonogram.NewBoard(2)
		board.Set(1, 0, nonogram.CellSet)
		for y := int32(0); y < 2; y++ {
			for x := int32(0); x < 2; x++ {
				if x == 1 && y == 0 {
					if got := board.Get(x, y); !got.IsSet() {
						t.Fatalf("expected set at [%v, %v], got %v", x, y, got)
					}
				} else {
					if got := board.Get(x, y); !got.IsUnknown() {
						t.Fatalf("expected unknown at [%v, %v], got %v", x, y, got)
					}
				}
			}
		}
	})

	t.Run("fail on index overflow", func(t *testing.T) {
		board := nonogram.NewBoard(2)
		if got := board.Get(2, 0); !got.IsUnknown() {
			t.Fatalf("expected unknown for x = 2, got %v", got)
		}
		if got := board.Get(0, 2); !got.IsUnknown() {
			t.Fatalf("expected unknown for y = 2, got %v", got)
		}
		if got := board.Get(-1, 0); !got.IsUnknown() {
			t.Fatalf("expected unknown for x = -1, got %v", got)
		}
		if got := board.Get(0, -1); !got.IsUnknown() {
			t.Fatalf("expected unknown for y = -1, got %v", got)
		}
	})

	t.Run("column", func(t *testing.T) {
		board := nonogram.NewBoard(4)
		board.Set(0, 0, nonogram.CellSet)
		board.Set(0, 2, nonogram.CellSet)
		board.Set(0, 3, nonogram.CellSet)
		board.Set(1, 1, nonogram.CellSet)
		board.Set(1, 2, nonogram.CellSet)
		board.Set(2, 1, nonogram.CellSet)
		board.Set(2, 3, nonogram.CellSet)
		t.Run("column 0", func(t *testing.T) {
			expected := []byte{1, 2}
			got := board.Column(0)
			for i, v := range got {
				if v != expected[i] {
					t.Fatalf("expected %v, got %v", expected, got)
				}
			}
		})
		t.Run("column 0 str", func(t *testing.T) {
			expected := "12"
			got := board.ColumnStr(0)
			if got != expected {
				t.Fatalf("expected %v, got %v", expected, got)
			}
		})
		t.Run("column 1", func(t *testing.T) {
			expected := []byte{2}
			got := board.Column(1)
			for i, v := range got {
				if v != expected[i] {
					t.Fatalf("expected %v, got %v", expected, got)
				}
			}
		})
		t.Run("column 1 str", func(t *testing.T) {
			expected := "2"
			got := board.ColumnStr(1)
			if got != expected {
				t.Fatalf("expected %v, got %v", expected, got)
			}
		})
		t.Run("column 3", func(t *testing.T) {
			expected := []byte{0}
			got := board.Column(3)
			for i, v := range got {
				if v != expected[i] {
					t.Fatalf("expected %v, got %v", expected, got)
				}
			}
		})
		t.Run("column 3 str", func(t *testing.T) {
			expected := "0"
			got := board.ColumnStr(3)
			if got != expected {
				t.Fatalf("expected %v, got %v", expected, got)
			}
		})
	})

	t.Run("line", func(t *testing.T) {
		board := nonogram.NewBoard(4)
		board.Set(0, 0, nonogram.CellSet)
		board.Set(2, 0, nonogram.CellSet)
		board.Set(3, 0, nonogram.CellSet)
		board.Set(1, 1, nonogram.CellSet)
		board.Set(2, 1, nonogram.CellSet)
		board.Set(1, 2, nonogram.CellSet)
		board.Set(3, 2, nonogram.CellSet)
		t.Run("line 0", func(t *testing.T) {
			expected := []byte{1, 2}
			got := board.Line(0)
			for i, v := range got {
				if v != expected[i] {
					t.Fatalf("expected %v, got %v", expected, got)
				}
			}
		})
		t.Run("line 0 str", func(t *testing.T) {
			expected := "12"
			got := board.LineStr(0)
			if got != expected {
				t.Fatalf("expected %v, got %v", expected, got)
			}
		})
		t.Run("line 1", func(t *testing.T) {
			expected := []byte{2}
			got := board.Line(1)
			for i, v := range got {
				if v != expected[i] {
					t.Fatalf("expected %v, got %v", expected, got)
				}
			}
		})
		t.Run("line 1 str", func(t *testing.T) {
			expected := "2"
			got := board.LineStr(1)
			if got != expected {
				t.Fatalf("expected %v, got %v", expected, got)
			}
		})
		t.Run("line 3", func(t *testing.T) {
			expected := []byte{0}
			got := board.Line(3)
			for i, v := range got {
				if v != expected[i] {
					t.Fatalf("expected %v, got %v", expected, got)
				}
			}
		})
		t.Run("line 3 str", func(t *testing.T) {
			expected := "0"
			got := board.LineStr(3)
			if got != expected {
				t.Fatalf("expected %v, got %v", expected, got)
			}
		})
	})

	t.Run("reveal column", func(t *testing.T) {
		board := nonogram.NewBoard(4)
		board.Set(0, 0, nonogram.CellSet)
		board.Set(0, 2, nonogram.CellSet)
		board.Set(0, 3, nonogram.CellSet)
		board.Set(1, 1, nonogram.CellSet)
		board.Set(1, 2, nonogram.CellSet)
		board.Set(2, 1, nonogram.CellSet)
		board.Set(2, 3, nonogram.CellSet)
		board.RevealColumn(0)
		if got := board.Get(0, 0); !got.IsSet() {
			t.Fatalf("expected 0, 0 to be set, got %v", got)
		}
		if got := board.Get(0, 1); !got.IsUnset() {
			t.Fatalf("expected 0, 1 to be unset, got %v", got)
		}
		if got := board.Get(0, 2); !got.IsSet() {
			t.Fatalf("expected 0, 2 to be set, got %v", got)
		}
		if got := board.Get(0, 3); !got.IsSet() {
			t.Fatalf("expected 0, 3 to be set, got %v", got)
		}
		if got := board.Get(1, 0); !got.IsUnknown() {
			t.Fatalf("expected 1, 0 to be untouched, got %v", got)
		}
	})

	t.Run("reveal line", func(t *testing.T) {
		board := nonogram.NewBoard(4)
		board.Set(0, 0, nonogram.CellSet)
		board.Set(2, 0, nonogram.CellSet)
		board.Set(3, 0, nonogram.CellSet)
		board.Set(1, 1, nonogram.CellSet)
		board.Set(2, 1, nonogram.CellSet)
		board.Set(1, 2, nonogram.CellSet)
		board.Set(3, 2, nonogram.CellSet)
		board.RevealLine(0)
		if got := board.Get(0, 0); !got.IsSet() {
			t.Fatalf("expected 0, 0 to be set, got %v", got)
		}
		if got := board.Get(1, 0); !got.IsUnset() {
			t.Fatalf("expected 1, 0 to be unset, got %v", got)
		}
		if got := board.Get(2, 0); !got.IsSet() {
			t.Fatalf("expected 2, 0 to be set, got %v", got)
		}
		if got := board.Get(3, 0); !got.IsSet() {
			t.Fatalf("expected 3, 0 to be set, got %v", got)
		}
		if got := board.Get(0, 1); !got.IsUnknown() {
			t.Fatalf("expected 0, 1 to be untouched, got %v", got)
		}
	})

	t.Run("eq", func(t *testing.T) {
		a := nonogram.NewBoard(4)
		a.Set(0, 0, nonogram.CellSet)
		a.Set(3, 0, nonogram.CellSet)
		a.Set(1, 1, nonogram.CellSet)
		a.Set(3, 1, nonogram.CellSet)
		a.Set(0, 3, nonogram.CellSet)
		a.Set(1, 3, nonogram.CellSet)
		a.Set(2, 3, nonogram.CellSet)
		a.Set(3, 3, nonogram.CellSet)
		b := nonogram.NewBoard(4)
		b.Set(1, 0, nonogram.CellSet)
		b.Set(3, 0, nonogram.CellSet)
		b.Set(0, 1, nonogram.CellSet)
		b.Set(3, 1, nonogram.CellSet)
		b.Set(0, 3, nonogram.CellSet)
		b.Set(1, 3, nonogram.CellSet)
		b.Set(2, 3, nonogram.CellSet)
		b.Set(3, 3, nonogram.CellSet)
		c := nonogram.NewBoard(4)
		c.Set(0, 3, nonogram.CellSet)
		c.Set(1, 3, nonogram.CellSet)
		c.Set(2, 3, nonogram.CellSet)
		c.Set(3, 3, nonogram.CellSet)
		if !a.Eq(b) {
			t.Fatalf("expected true, got false")
		}
		if a.Eq(c) {
			t.Fatalf("expected false, got true")
		}
	})
}
