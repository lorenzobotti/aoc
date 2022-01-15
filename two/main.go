package main

import (
	"fmt"
	"io/ioutil"
	"os"
	"strconv"
	"strings"
)

func main() {
	input, err := ioutil.ReadAll(os.Stdin)
	handle(err)
	// input := readAllInput()

	lines := strings.Split(string(input), "\n")

	aim, depth, distance := 0, 0, 0

	for _, line := range lines {
		elems := strings.Split(line, " ")
		instruction := elems[0]
		amountRaw := elems[1]
		amount, err := strconv.Atoi(amountRaw)
		handle(err)

		switch instruction {
		case "forward":
			distance += amount
			depth += aim * amount
		case "up":
			aim -= amount
		case "down":
			aim += amount
		default:
			panic("unexpected instruction " + instruction)
		}
	}

	fmt.Println("depth:", depth)
	fmt.Println("distance:", distance)
	fmt.Println("depth * distance:", depth*distance)
}

func readAllInput() []byte {
	file, err := os.Open("sample_input.txt")
	handle(err)
	contents, err := ioutil.ReadAll(file)
	handle(err)
	return contents
}

func handle(err interface{}) {
	if err != nil {
		panic(err)
	}
}
