package nonogram

type Cell byte

const (
	CellUnknown Cell = iota
	CellUnset
	CellSet
)

func (c Cell) IsUnknown() bool { return c == CellUnknown }
func (c Cell) IsSet() bool     { return c == CellSet }
func (c Cell) IsUnset() bool   { return c == CellUnset }
func (c Cell) IsFilled() bool  { return c == CellSet || c == CellUnset }

func (c Cell) String() string {
	switch c {
	case CellUnknown:
		return "?"
	case CellUnset:
		return "X"
	case CellSet:
		return "O"
	}
	panic("not a cell")
}
