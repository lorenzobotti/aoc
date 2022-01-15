package main

import (
	"strings"
)

type inputLine struct {
	patterns [10]displayed
	output   [4]displayed
}

func (il inputLine) one() displayed {
	oneLength := len(digits[1].segments())
	for _, pattern := range il.patterns {
		patLength := len(pattern.segments())
		if patLength == oneLength {
			return pattern
		}
	}

	panic("no 1 found")
}

func (il inputLine) four() displayed {
	for _, pattern := range il.patterns {
		if len(pattern.segments()) == len(digits[4].segments()) {
			return pattern
		}
	}

	panic("no 4 found")
}

func (il inputLine) seven() displayed {
	for _, pattern := range il.patterns {
		if len(pattern.segments()) == len(digits[7].segments()) {
			return pattern
		}
	}

	panic("no 7 found")
}

func (il inputLine) eight() displayed {
	for _, pattern := range il.patterns {
		if len(pattern.segments()) == len(digits[8].segments()) {
			return pattern
		}
	}

	panic("no 8 found")
}

func (il inputLine) three() displayed {
	fiveSegmentDigits := il.withNSegments(5)

	// these are the 2, 3 and 5 but we don't know which is which
	digitA := fiveSegmentDigits[0]
	digitB := fiveSegmentDigits[1]
	digitC := fiveSegmentDigits[2]

	if len(digitA.similarSegments(digitB)) == 3 {
		return digitC
	}
	if len(digitA.similarSegments(digitC)) == 3 {
		return digitB
	}
	if len(digitB.similarSegments(digitC)) == 3 {
		return digitA
	}

	panic("no 3 found")
}

func (il inputLine) nine(three, eight displayed) displayed {
	for _, pat := range il.patterns {
		if pat == three || pat == eight {
			continue
		}

		if pat.union(three) == three {
			return pat
		}
	}

	panic("no 9 found")
}

func (il inputLine) twoAndFive(three, nine displayed) (displayed, displayed) {
	two := displayed{}
	five := displayed{}
	defaultDisplayed := displayed{}

	for _, pat := range il.withNSegments(5) {
		if pat == three {
			continue
		}

		unionSegments := len(pat.union(nine).segments())

		if unionSegments == 5 {
			five = pat
		} else if unionSegments == 4 {
			two = pat
		} else {
			panic("unknown pattern")
		}
	}

	if two == defaultDisplayed || five == defaultDisplayed {
		panic("no 2 and 5 found")
	}

	return two, five
}

func (il inputLine) zeroAndSix(seven, nine displayed) (displayed, displayed) {
	zero, six, defaultDisplayed := displayed{}, displayed{}, displayed{}

	for _, pat := range il.withNSegments(6) {
		if pat == nine {
			continue
		}

		unionSegments := len(pat.union(seven).segments())
		if unionSegments == 3 {
			zero = pat
		} else if unionSegments == 2 {
			six = pat
		} else {
			panic("unknown pattern")
		}
	}

	if zero == defaultDisplayed || six == defaultDisplayed {
		panic("can't find 0 and 6")
	}

	return zero, six
}

func (il inputLine) correctDigits() []displayed {
	one := il.one()
	four := il.four()
	seven := il.seven()
	eight := il.eight()
	three := il.three()
	nine := il.nine(three, eight)
	two, five := il.twoAndFive(three, nine)
	zero, six := il.zeroAndSix(seven, nine)

	return []displayed{zero, one, two, three, four, five, six, seven, eight, nine}
}

func (il inputLine) whichIsIt(correct []displayed, which displayed) int {
	for i, pat := range correct {
		if pat == which {
			return i
		}
	}

	return -1
}

func (il inputLine) solve() int {
	correct := il.correctDigits()
	out := 0

	for _, pat := range il.output {
		digit := il.whichIsIt(correct, pat)
		if digit == -1 {
			return -1
		} else {
			out = out*10 + digit
		}
	}

	return out
}

func (il inputLine) withNSegments(n int) []displayed {
	out := []displayed{}
	for _, pattern := range il.patterns {
		if len(pattern.segments()) == n {
			out = append(out, pattern)
		}
	}

	return out
}

func parseInputLine(input string) (inputLine, error) {
	parsed := inputLine{}
	patternsRaw, outputRaw, _ := splitInTwoAndTrim(input, "|")

	patternsSliceRaw := strings.Split(patternsRaw, " ")
	outputSliceRaw := strings.Split(outputRaw, " ")

	for i, patternRaw := range patternsSliceRaw {
		patternRaw = strings.TrimSpace(patternRaw)
		disp := displayedFromString(patternRaw)
		parsed.patterns[i] = disp
	}

	for i, outRaw := range outputSliceRaw {
		outRaw = strings.TrimSpace(outRaw)
		disp := displayedFromString(outRaw)
		parsed.output[i] = disp
	}

	return parsed, nil
}

func splitInTwo(input, separator string) (string, string, bool) {
	oneAndTwo := strings.SplitN(input, separator, 2)

	one := oneAndTwo[0]
	two := oneAndTwo[1]
	ok := true

	if one == input {
		ok = false
	}

	return one, two, ok
}

func splitInTwoAndTrim(input, separator string) (string, string, bool) {
	one, two, ok := splitInTwo(input, separator)
	return strings.TrimSpace(one), strings.TrimSpace(two), ok
}
