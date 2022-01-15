package main

type Coord struct {
	row int
	col int
}

func (s Squid) Direction() Coord {
	switch s {
	case East:
		return Coord{row: 0, col: 1}
	case South:
		return Coord{row: 1, col: 0}
	default:
		return Coord{row: 0, col: 0}
	}
}

func (s Squid) Moves() bool {
	return s == East || s == South
}

func (c Coord) Plus(other Coord) Coord {
	return Coord{c.row + other.row, c.col + other.col}
}
