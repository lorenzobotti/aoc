package main

import (
	"testing"

	"github.com/matryer/is"
)

func TestOpposite(t *testing.T) {
	testCases := []struct {
		line int
		pos  int
		exp  int
	}{
		{4, 3, 3},
		{4, 5, 3},
		{8, 5, 5},
		{8, 10, 6},
		{8, 8, 8},
		{8, 15, 1},
		{8, 17, 0},
		{5, 6, 4},
	}

	is := is.New(t)

	for _, tC := range testCases {
		is.Equal(opposite(tC.pos, tC.line), tC.exp)
	}
}
