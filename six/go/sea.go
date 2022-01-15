package main

import (
	"strconv"
	"strings"
)

type BetterSea struct {
	lanterns [9]int
}

func ParseBetterSea(input string) (BetterSea, error) {
	sea := BetterSea{}

	input = strings.TrimSpace(input)
	nums := strings.Split(input, ",")

	for _, numRaw := range nums {
		num, err := strconv.Atoi(numRaw)
		if err != nil {
			return sea, err
		}

		sea.lanterns[num] += 1
	}

	return sea, nil
}

func (s *BetterSea) Tick() {
	nextGen := [9]int{}

	for age := 8; age >= 1; age-- {
		nextGen[age-1] = s.lanterns[age]
	}

	nextGen[lanternPuberty+lanternCycle-1] = s.lanterns[0]
	nextGen[lanternCycle-1] += s.lanterns[0]
	s.lanterns = nextGen
}

func (s BetterSea) Population() int {
	pop := 0
	for _, num := range s.lanterns {
		pop += num
	}
	return pop
}
