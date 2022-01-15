package main

import (
	"fmt"
	"strconv"
	"strings"
)

func parse(input string) (sheet, []fold, error) {
	dots, folds := splitInTwo(input, "\n\n")

	outSheet := newSheet(20, 20)
	for _, line := range strings.Split(dots, "\n") {
		columnRaw, rowRaw := splitInTwo(line, ",")
		row, err := strconv.Atoi(rowRaw)
		if err != nil {
			return sheet{}, nil, err
		}
		column, err := strconv.Atoi(columnRaw)
		if err != nil {
			return sheet{}, nil, err
		}

		outSheet.addDot(row, column)
	}

	outSheet.trim()

	outFolds := []fold{}
	for _, line := range strings.Split(folds, "\n") {
		f, err := parseFold(line)
		if err != nil {
			return sheet{}, nil, err
		}

		outFolds = append(outFolds, f)
	}

	return outSheet, outFolds, nil
}

func parseAxis(input string) (axis, error) {
	switch input {
	case "x":
		fallthrough
	case "X":
		return x, nil
	case "y":
		fallthrough
	case "Y":
		return y, nil
	default:
		return 0, fmt.Errorf("'%s' is not a valid axis", input)
	}
}

func (a axis) String() string {
	switch a {
	case x:
		return "x"
	case y:
		return "y"
	default:
		return "???"
	}
}

func (f fold) String() string {
	return fmt.Sprintf("fold along %s=%d", f.axis.String(), f.line)
}

func parseFold(input string) (fold, error) {
	prefix := "fold along "
	if !strings.HasPrefix(input, prefix) {
		return fold{}, fmt.Errorf("line '%s', wrong", input)
	}

	input = strings.TrimPrefix(input, prefix)
	axisRaw, lineRaw := splitInTwo(input, "=")
	axis, err := parseAxis(axisRaw)
	if err != nil {
		return fold{}, err
	}
	line, err := strconv.Atoi(lineRaw)
	if err != nil {
		return fold{}, err
	}

	return fold{axis, line}, nil
}
