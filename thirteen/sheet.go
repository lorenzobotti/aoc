package main

import (
	"fmt"
	"strings"
)

type sheet struct {
	dots [][]bool

	width  int
	height int
}

func newSheet(width, height int) sheet {
	return sheet{
		dots:   multiDimSlice(width, height),
		width:  width,
		height: height,
	}
}

func multiDimSlice(width, height int) [][]bool {
	out := make([][]bool, height)
	for h := range out {
		out[h] = make([]bool, width)
	}
	return out
}

func transferMultidimSlice(old [][]bool, width, height int) [][]bool {
	newSlice := multiDimSlice(width, height)

	for r, row := range old {
		if r >= len(newSlice) {
			break
		}

		for c, column := range row {
			if c >= len(newSlice[0]) {
				break
			}

			newSlice[r][c] = column
		}
	}

	return newSlice
}

func (s *sheet) addDot(row, column int) {
	if s.width <= column || s.height <= row {
		newHeight := max(s.height, row+1)
		newWidth := max(s.width, column+1)
		s.dots = transferMultidimSlice(s.dots, newWidth, newHeight)
		s.height = newHeight
		s.width = newWidth

		fmt.Println(s.width, s.height)
	}

	s.dots[row][column] = true
}

func (s sheet) dotCount() int {
	count := 0

	for _, row := range s.dots {
		for _, dot := range row {
			if dot {
				count += 1
			}
		}
	}

	return count
}

func (s *sheet) trim() {
	furthestRow := 0
	furthestColumn := 0

	for r, row := range s.dots {
		furthestColumnInThisRow := 0
		for c, column := range row {
			if column {
				furthestColumnInThisRow = c
				furthestRow = r
			}
		}

		furthestColumn = max(furthestColumnInThisRow, furthestColumn)
	}

	s.dots = transferMultidimSlice(s.dots, furthestColumn+1, furthestRow+1)
}

func (s sheet) String() string {
	out := strings.Builder{}

	for _, row := range s.dots {
		for _, dot := range row {
			if dot {
				out.WriteRune('#')
			} else {
				out.WriteRune(' ')
			}
		}
		out.WriteRune('\n')
	}

	return out.String()
}
