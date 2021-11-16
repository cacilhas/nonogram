package tests

import (
	"testing"

	"github.com/cacilhas/nonogram/nonogram"
)

func TestCell(t *testing.T) {
	t.Run("CellUnknown", func(t *testing.T) {
		if !nonogram.CellUnknown.IsUnknown() {
			t.Fatal("CellUnknown expected to be unknown")
		}
		if nonogram.CellUnknown.IsSet() {
			t.Fatal("CellUnknown expected to be not set")
		}
		if nonogram.CellUnknown.IsUnset() {
			t.Fatal("CellUnknown expected to be not unset")
		}
		if nonogram.CellUnknown.IsFilled() {
			t.Fatal("CellUnknown expected to be not filled")
		}
		if got := nonogram.CellUnknown.String(); got != "?" {
			t.Fatalf("CellUnknown expected to be ?, got %v", got)
		}
	})
	t.Run("CellUnset", func(t *testing.T) {
		if !nonogram.CellUnset.IsUnset() {
			t.Fatal("CellUnset expected to be unset")
		}
		if nonogram.CellUnset.IsSet() {
			t.Fatal("CellUnset expected to be not set")
		}
		if nonogram.CellUnset.IsUnknown() {
			t.Fatal("CellUnset expected to be not unknown")
		}
		if !nonogram.CellUnset.IsFilled() {
			t.Fatal("CellUnset expected to be filled")
		}
		if got := nonogram.CellUnset.String(); got != "X" {
			t.Fatalf("CellUnset expected to be X, got %v", got)
		}
	})
	t.Run("CellSet", func(t *testing.T) {
		if !nonogram.CellSet.IsSet() {
			t.Fatal("CellSet expected to be set")
		}
		if nonogram.CellSet.IsUnset() {
			t.Fatal("CellSet expected to be not unset")
		}
		if nonogram.CellSet.IsUnknown() {
			t.Fatal("CellSet expected to be not unknown")
		}
		if !nonogram.CellSet.IsFilled() {
			t.Fatal("CellSet expected to be filled")
		}
		if got := nonogram.CellSet.String(); got != "O" {
			t.Fatalf("CellSet expected to be X, got %v", got)
		}
	})
}
