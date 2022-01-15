package main

import "fmt"

func mostCommonAt(numbers []string, i int) (rune, error) {
	zeroes := 0
	ones := 0

	for _, num := range numbers {
		digit := num[i]
		switch digit {
		case '0':
			zeroes += 1
		case '1':
			ones += 1
		default:
			return 0, fmt.Errorf("unknown digit: '%c'", digit)
		}
	}

	if zeroes >= ones {
		return '0', nil
	} else {
		return '1', nil
	}
}
