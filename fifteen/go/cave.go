package main

import "strings"

type (
	cave struct {
		places map[coord]*place

		topLeft     *place
		bottomRight *place
	}

	place struct {
		risk int

		right *place
		left  *place
		up    *place
		down  *place
	}

	coord struct {
		row    int
		column int
	}
)

func parseCave(input string) cave {
	out := cave{places: map[coord]*place{}}
	lines := strings.Split(input, "\n")

	// parse each digit
	for row, line := range lines {
		for column, riskRaw := range line {
			c := coord{row, column}
			risk := parseDigit(riskRaw)

			out.places[c] = &place{risk: risk}
		}
	}

	// fill in start and end
	lastRow := len(lines) - 1
	lastColumn := len(lines[lastRow]) - 1

	out.topLeft = out.places[coord{0, 0}]
	out.bottomRight = out.places[coord{lastRow, lastColumn}]

	// connect the places
	for location, pl := range out.places {
		left, ok := out.places[location.left()]
		if ok {
			pl.left = left
		}

		right, ok := out.places[location.right()]
		if ok {
			pl.right = right
		}

		up, ok := out.places[location.up()]
		if ok {
			pl.up = up
		}

		down, ok := out.places[location.down()]
		if ok {
			pl.down = down
		}
	}

	return out
}

func parseDigit(digit rune) int {
	return int(digit - '0')
}

func (c coord) up() coord {
	return coord{row: c.row - 1, column: c.column}
}

func (c coord) down() coord {
	return coord{row: c.row + 1, column: c.column}
}

func (c coord) right() coord {
	return coord{row: c.row, column: c.column + 1}
}

func (c coord) left() coord {
	return coord{row: c.row, column: c.column - 1}
}
