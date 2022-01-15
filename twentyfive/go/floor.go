package main

import (
	"strings"
)

type Floor struct {
	squids []Squid
	rows   int
	cols   int
}

func (f *Floor) Step(turn Squid) {
	nextGen := f.Clone()

	for row := 0; row < f.rows; row++ {
		for col := 0; col < f.cols; col++ {

			c := Coord{row, col}
			sq := f.At(c)

			if sq == turn {

				destinationCoord := f.Wrap(c.Plus(sq.Direction()))
				destination := f.At(destinationCoord)

				// fmt.Println("found a squid at", c)
				// fmt.Println("destination:", destinationCoord)

				if destination == Nothing {
					// fmt.Println("destination empty")

					nextGen.Set(destinationCoord, sq)
					nextGen.Set(c, Nothing)
				}
			}
		}
	}

	f.squids = nextGen.squids
}

func (f *Floor) FullStep() bool {
	before := make([]Squid, len(f.squids))
	copy(before, f.squids)

	f.Step(East)
	f.Step(South)

	return squidSliceEqual(before, f.squids)
}

func (f *Floor) UntilGridLock() int {
	for i := 1; ; i++ {
		if f.FullStep() {
			return i
		}
	}
}

func (f Floor) Clone() Floor {
	nextSlice := make([]Squid, len(f.squids))
	copy(nextSlice, f.squids)

	return Floor{
		squids: nextSlice,
		rows:   f.rows,
		cols:   f.cols,
	}
}

func (f Floor) Wrap(c Coord) Coord {
	return Coord{
		c.row % f.rows,
		c.col % f.cols,
	}
}

func (f Floor) At(c Coord) Squid {
	i := c.row*f.cols + c.col
	return f.squids[i]
}

func (f Floor) Set(c Coord, sq Squid) {
	c = f.Wrap(c)
	i := c.row*f.cols + c.col

	f.squids[i] = sq
}

func (f Floor) String() string {
	out := strings.Builder{}

	for row := 0; row < f.rows; row++ {
		for col := 0; col < f.cols; col++ {
			c := Coord{row, col}
			dest := f.At(c)

			out.WriteRune(rune(dest))
		}
		out.WriteRune('\n')
	}

	return out.String()
}

func (f Floor) DebugString() string {
	out := strings.Builder{}

	out.WriteRune(' ')

	for col := 0; col < f.cols; col++ {
		out.WriteRune('0' + rune(col%10))
	}

	for row := 0; row < f.rows; row++ {
		out.WriteRune('0' + rune(row%10))

		for col := 0; col < f.cols; col++ {
			c := Coord{row, col}
			dest := f.At(c)

			out.WriteRune(rune(dest))
		}
		out.WriteRune('\n')
	}

	return out.String()
}

func ParseFloor(input string) Floor {
	lines := strings.Split(input, "\n")
	lineLength := 0
	lineCount := len(lines)

	squids := []Squid{}

	for _, line := range lines {
		lineLength = len(line)

		for _, r := range line {
			squids = append(squids, Squid(r))
		}
	}

	return Floor{
		squids: squids,
		rows:   lineCount,
		cols:   lineLength,
	}
}

func squidSliceEqual(a, b []Squid) bool {
	if len(a) != len(b) {
		return false
	}
	for i, v := range a {
		if v != b[i] {
			return false
		}
	}
	return true
}
