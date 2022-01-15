package main

import (
	"fmt"
	"io/ioutil"
	"math"
	"os"
)

func main() {
	target, err := parseArea(readIn())

	if err != nil {
		panic(err)
	}

	highest := math.MinInt
	// highestX := math.MinInt
	// highestY := math.MinInt
	possibleThrows := []Coord{}

	for x := 0; x <= target.xRight; x++ {
		for y := -2000; y < 2000; y++ {
			probe := Probe{
				pos:    Coord{0, 0},
				xSpeed: x,
				ySpeed: y,
			}

			if hits, height := probe.Hits(target); hits {
				possibleThrows = append(possibleThrows, Coord{x, y})

				if height > highest {
					highest = height
					// highestX = x
					// highestY = y
				}
			}
		}
	}

	fmt.Println("highest elevation:", highest)
	fmt.Println("possible trajectories:", len(possibleThrows))
}

func readIn() string {
	contents, err := ioutil.ReadAll(os.Stdin)
	if err != nil {
		panic(err)
	}

	return string(contents)
}
