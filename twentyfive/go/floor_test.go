package main

import (
	"strings"
	"testing"
	"unicode"
)

func TestOneMover(t *testing.T) {
	tests := []struct {
		start    string
		expected string
	}{
		{
			removeIndentation(`
......
..v...
......`),

			removeIndentation(`
......
......
..v...`),
		},
	}

	for _, test := range tests {
		t.Run("test", func(t *testing.T) {
			field := ParseFloor(test.start)

			field.Step(East)
			field.Step(South)

			if removeSpace(field.String()) != removeSpace(test.expected) {
				t.Fail()
			}
		})
	}
}

func TestMove(t *testing.T) {
	testCases := []struct {
		desc     string
		one      Coord
		two      Coord
		expected Coord
	}{
		{
			desc:     "one column down",
			one:      Coord{3, 3},
			two:      South.Direction(),
			expected: Coord{4, 3},
		},
		{
			desc:     "one row right",
			one:      Coord{3, 3},
			two:      East.Direction(),
			expected: Coord{3, 4},
		},
		{
			desc:     "one down, one right",
			one:      Coord{3, 3},
			two:      Coord{1, 1},
			expected: Coord{4, 4},
		},
	}
	for _, tC := range testCases {
		t.Run(tC.desc, func(t *testing.T) {
			if tC.one.Plus(tC.two) != tC.expected {
				t.Fail()
			}
		})
	}
}

func TestWrap(t *testing.T) {
	testCases := []struct {
		desc     string
		rows     int
		cols     int
		c        Coord
		expected Coord
	}{
		{
			desc: "one too far down",
			rows: 3, cols: 3,
			c:        Coord{3, 1},
			expected: Coord{0, 1},
		},
		{
			desc: "one too far right",
			rows: 3, cols: 3,
			c:        Coord{1, 3},
			expected: Coord{1, 0},
		},
	}
	for _, tC := range testCases {
		t.Run(tC.desc, func(t *testing.T) {
			field := Floor{
				rows: tC.rows,
				cols: tC.cols,
			}

			if field.Wrap(tC.c) != tC.expected {
				t.Fail()
			}
		})
	}
}

func removeIndentation(inp string) string {
	out := strings.Builder{}

	for i, r := range inp {
		if r == '\n' && i != 0 {
			out.WriteRune(r)
		} else if unicode.IsSpace(r) {
			continue
		} else {
			out.WriteRune(r)
		}
	}

	return out.String()
}

func removeSpace(inp string) string {
	out := strings.Builder{}

	for _, r := range inp {
		if unicode.IsSpace(r) {
			continue
		} else {
			out.WriteRune(r)
		}
	}

	return out.String()
}
