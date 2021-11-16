package tests

import (
	"math/rand"
	"testing"

	"github.com/cacilhas/nonogram/nonogram"
)

func TestGame(t *testing.T) {
	t.Run("new game", func(t *testing.T) {
		game := getGame()
		if game.IsDone() {
			t.Fatalf("expected game to be undone")
		}

		t.Run("reference", func(t *testing.T) {
			reference := game.Reference()
			if got := reference.Get(0, 0); !got.IsUnset() {
				t.Fatalf("expected unset at [0, 0], got %v", got)
			}
			if got := reference.Get(1, 0); !got.IsUnset() {
				t.Fatalf("expected unset at [1, 0], got %v", got)
			}
			if got := reference.Get(2, 0); !got.IsUnset() {
				t.Fatalf("expected unset at [2, 0], got %v", got)
			}
			if got := reference.Get(3, 0); !got.IsUnset() {
				t.Fatalf("expected unset at [3, 0], got %v", got)
			}
			if got := reference.Get(0, 1); !got.IsUnset() {
				t.Fatalf("expected unset at [0, 1], got %v", got)
			}
			if got := reference.Get(1, 1); !got.IsUnset() {
				t.Fatalf("expected unset at [1, 1], got %v", got)
			}
			if got := reference.Get(2, 1); !got.IsSet() {
				t.Fatalf("expected set at [2, 1], got %v", got)
			}
			if got := reference.Get(3, 1); !got.IsSet() {
				t.Fatalf("expected set at [3, 1], got %v", got)
			}
			if got := reference.Get(0, 2); !got.IsSet() {
				t.Fatalf("expected set at [0, 2], got %v", got)
			}
			if got := reference.Get(1, 2); !got.IsUnset() {
				t.Fatalf("expected unset at [1, 2], got %v", got)
			}
			if got := reference.Get(2, 2); !got.IsUnset() {
				t.Fatalf("expected unset at [2, 2], got %v", got)
			}
			if got := reference.Get(3, 2); !got.IsUnset() {
				t.Fatalf("expected unset at [3, 2], got %v", got)
			}
			if got := reference.Get(0, 3); !got.IsSet() {
				t.Fatalf("expected set at [0, 3], got %v", got)
			}
			if got := reference.Get(1, 3); !got.IsUnset() {
				t.Fatalf("expected unset at [1, 3], got %v", got)
			}
			if got := reference.Get(2, 3); !got.IsUnset() {
				t.Fatalf("expected unset at [2, 3], got %v", got)
			}
			if got := reference.Get(3, 3); !got.IsUnset() {
				t.Fatalf("expected unset at [3, 3], got %v", got)
			}
		})

		t.Run("round", func(t *testing.T) {
			round := game.Round()
			if got := round.Get(0, 0); !got.IsUnset() {
				t.Fatalf("expected unset at [0, 0], got %v", got)
			}
			if got := round.Get(1, 0); !got.IsUnset() {
				t.Fatalf("expected unset at [1, 0], got %v", got)
			}
			if got := round.Get(2, 0); !got.IsUnknown() {
				t.Fatalf("expected unknown at [2, 0], got %v", got)
			}
			if got := round.Get(3, 0); !got.IsUnset() {
				t.Fatalf("expected unset at [3, 0], got %v", got)
			}
			if got := round.Get(0, 1); !got.IsUnset() {
				t.Fatalf("expected unset at [0, 1], got %v", got)
			}
			if got := round.Get(1, 1); !got.IsUnset() {
				t.Fatalf("expected unset at [1, 1], got %v", got)
			}
			if got := round.Get(2, 1); !got.IsUnknown() {
				t.Fatalf("expected unknown at [2, 1], got %v", got)
			}
			if got := round.Get(3, 1); !got.IsUnknown() {
				t.Fatalf("expected unknown at [3, 1], got %v", got)
			}
			if got := round.Get(0, 2); !got.IsUnknown() {
				t.Fatalf("expected unknown at [0, 2], got %v", got)
			}
			if got := round.Get(1, 2); !got.IsUnknown() {
				t.Fatalf("expected unknown at [1, 2], got %v", got)
			}
			if got := round.Get(2, 2); !got.IsUnknown() {
				t.Fatalf("expected unknown at [2, 2], got %v", got)
			}
			if got := round.Get(3, 2); !got.IsUnset() {
				t.Fatalf("expected unset at [3, 2], got %v", got)
			}
			if got := round.Get(0, 3); !got.IsUnknown() {
				t.Fatalf("expected unknown at [0, 3], got %v", got)
			}
			if got := round.Get(1, 3); !got.IsUnset() {
				t.Fatalf("expected unset at [1, 3], got %v", got)
			}
			if got := round.Get(2, 3); !got.IsUnknown() {
				t.Fatalf("expected unknown at [2, 3], got %v", got)
			}
			if got := round.Get(3, 3); !got.IsUnset() {
				t.Fatalf("expected unset at [3, 3], got %v", got)
			}
		})
	})

	t.Run("check", func(t *testing.T) {
		game := getGame()
		round := game.Round()
		round.Set(0, 2, nonogram.CellSet)
		round.Set(0, 3, nonogram.CellSet)
		game.Check(0, 2)

		if got := round.Get(0, 0); !got.IsUnset() {
			t.Fatalf("expected unset at [0, 0], got %v", got)
		}
		if got := round.Get(1, 0); !got.IsUnset() {
			t.Fatalf("expected unset at [1, 0], got %v", got)
		}
		if got := round.Get(2, 0); !got.IsUnknown() {
			t.Fatalf("expected unknown at [2, 0], got %v", got)
		}
		if got := round.Get(3, 0); !got.IsUnset() {
			t.Fatalf("expected unset at [3, 0], got %v", got)
		}
		if got := round.Get(0, 1); !got.IsUnset() {
			t.Fatalf("expected unset at [0, 1], got %v", got)
		}
		if got := round.Get(1, 1); !got.IsUnset() {
			t.Fatalf("expected unset at [1, 1], got %v", got)
		}
		if got := round.Get(2, 1); !got.IsUnknown() {
			t.Fatalf("expected unknown at [2, 1], got %v", got)
		}
		if got := round.Get(3, 1); !got.IsUnknown() {
			t.Fatalf("expected unknown at [3, 1], got %v", got)
		}
		if got := round.Get(0, 2); !got.IsSet() {
			t.Fatalf("expected set at [0, 2], got %v", got)
		}
		if got := round.Get(1, 2); !got.IsUnset() {
			t.Fatalf("expected unset at [1, 2], got %v", got)
		}
		if got := round.Get(2, 2); !got.IsUnset() {
			t.Fatalf("expected unset at [2, 2], got %v", got)
		}
		if got := round.Get(3, 2); !got.IsUnset() {
			t.Fatalf("expected unset at [3, 2], got %v", got)
		}
		if got := round.Get(0, 3); !got.IsSet() {
			t.Fatalf("expected set at [0, 3], got %v", got)
		}
		if got := round.Get(1, 3); !got.IsUnset() {
			t.Fatalf("expected unset at [1, 3], got %v", got)
		}
		if got := round.Get(2, 3); !got.IsUnknown() {
			t.Fatalf("expected unknown at [2, 3], got %v", got)
		}
		if got := round.Get(3, 3); !got.IsUnset() {
			t.Fatalf("expected unset at [3, 3], got %v", got)
		}
	})

	t.Run("is done", func(t *testing.T) {
		game := getGame()
		reference := game.Reference()
		round := game.Round()

		for y := int32(0); y < 4; y++ {
			for x := int32(0); x < 4; x++ {
				round.Set(x, y, reference.Get(x, y))
			}
		}

		if !game.IsDone() {
			t.Fatalf("expected game to be done")
		}
	})
}

func getGame() nonogram.Game {
	rand.Seed(1)
	return nonogram.NewGame(4, 0.3, 0.5)
}
