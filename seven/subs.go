package main

import (
	"math"
	"strconv"
	"strings"
)

type subs struct {
	subs map[int]int

	max int
	min int
}

func newSub(input string) subs {
	numsRaw := strings.Split(input, ",")
	out := subs{
		subs: map[int]int{},
		min:  math.MaxInt,
		max:  math.MinInt,
	}

	for _, numRaw := range numsRaw {
		num, err := strconv.Atoi(strings.TrimSpace(numRaw))
		if err != nil {
			panic(err)
		}

		out.subs[num] += 1
		if num > out.max {
			out.max = num
		}
		if num < out.min {
			out.min = num
		}

	}

	return out
}

func (s subs) fuelToGetTo(spot int) int {
	fuel := 0
	for place, crabs := range s.subs {
		fuel += movementCost(difference(place, spot)) * crabs
	}

	return fuel
}

func (s subs) mostEfficientSpot() (int, int) {
	spot := s.min
	minFuel := math.MaxInt
	for i := s.min; i <= s.max; i++ {
		fuel := s.fuelToGetTo(i)
		if fuel < minFuel {
			minFuel = fuel
			spot = i
		}
	}

	return spot, minFuel
}

func difference(a, b int) int {
	if a > b {
		return a - b
	}

	return b - a
}

func movementCost(distance int) int {
	out := 0
	for i := 1; i <= distance; i++ {
		out += i
	}
	return out
}
