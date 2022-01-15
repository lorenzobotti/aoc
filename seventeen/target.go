package main

import (
	"regexp"
	"strconv"
)

type Area struct {
	xLeft   int
	xRight  int
	yTop    int
	yBottom int
}

func (a Area) Hit(c Coord) bool {
	return c.x >= a.xLeft && c.x <= a.xRight && c.y >= a.yBottom && c.y <= a.yTop
}

var areaRegex = regexp.MustCompile(`target area: x=([-\d]*)\.\.([-\d]*), y=([-\d]*)..([-\d]*)`)

func parseArea(input string) (Area, error) {
	res := areaRegex.FindStringSubmatch(input)
	out := Area{}

	var err error
	out.xLeft, err = strconv.Atoi(res[1])
	if err != nil {
		return out, err
	}
	out.xRight, err = strconv.Atoi(res[2])
	if err != nil {
		return out, err
	}
	out.yBottom, err = strconv.Atoi(res[3])
	if err != nil {
		return out, err
	}
	out.yTop, err = strconv.Atoi(res[4])
	if err != nil {
		return out, err
	}

	return out, err
}
