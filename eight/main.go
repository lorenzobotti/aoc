package main

import (
	"fmt"
	"io/ioutil"
	"os"
	"strings"
)

func main() {
	// fmt.Println("two")
	// fmt.Println(digits[2])

	// fmt.Println("nine")
	// fmt.Println(digits[9])

	// fmt.Println("union")
	// fmt.Println(digits[2].union(digits[9]))

	// return
	// input := regularInputLine()

	// input := "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf"
	// line, _ := parseInputLine(input)

	// fmt.Println(line.nineAndZero())

	// fmt.Println(line.three().compactString())
	// three := line.three()
	// eight := line.eight()
	// nine := line.nine(three, eight)
	// fmt.Println(line.twoAndFive(three, nine))
	// for i, pat := range line.correctDigits() {
	// 	fmt.Println(i)
	// 	fmt.Println(pat)
	// 	fmt.Println("")
	// }

	input := readInput()
	solution := 0
	for _, lineRaw := range strings.Split(input, "\n") {
		line, err := parseInputLine(lineRaw)
		if err != nil {
			panic(err)
		}

		solution += line.solve()
	}

	fmt.Println(solution)

	// return
	// input := strings.Split(readInput(), "\n")
	// lines := []inputLine{}
	// for _, lineRaw := range input {
	// 	line, err := parseInputLine(lineRaw)
	// 	if err != nil {
	// 		panic(err)
	// 	}
	// 	lines = append(lines, line)
	// }

	// counter := 0
	// for _, line := range lines {
	// 	for _, out := range line.output {
	// 		if out.uniqueSegmentCount() {
	// 			counter++
	// 		}
	// 	}
	// }

	// fmt.Println(counter)
}

func readInput() string {
	contents, err := ioutil.ReadAll(os.Stdin)
	if err != nil {
		panic(err)
	}
	return string(contents)
}

func regularInputLine() string {
	build := strings.Builder{}

	for _, dig := range digits {
		for _, r := range dig.segments() {
			build.WriteRune(r)
		}
		build.WriteRune(' ')
	}

	build.WriteRune('|')
	for _, dig := range []displayed{digits[6], digits[4], digits[3], digits[2]} {
		build.WriteRune(' ')
		for _, r := range dig.segments() {
			build.WriteRune(r)
		}
	}

	return build.String()
}
