package main

import "fmt"

type fold struct {
	axis axis
	line int
}

type axis int

const (
	x axis = iota
	y
)

func (s *sheet) fold(f fold) {
	fmt.Println(f)
	if f.axis == x {
		s.foldX(f.line)
	} else {
		s.foldY(f.line)
	}
}

func (s *sheet) foldY(line int) {
	for r, row := range s.dots {
		for c, dot := range row {
			if !dot {
				continue
			}

			s.dots[r][c] = false
			s.dots[opposite(r, line)][c] = true
		}
	}

	s.trim()
}

func (s *sheet) foldX(line int) {
	for r, row := range s.dots {
		for c, dot := range row {
			if !dot {
				continue
			}

			s.dots[r][c] = false
			s.dots[r][opposite(c, line)] = true
		}
	}

	s.trim()
}

func opposite(point, line int) int {
	if point < line {
		return point
	} else {
		opp := line*2 - point
		return max(0, opp)
	}
}
