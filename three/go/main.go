package main

import (
	"fmt"
	"io/ioutil"
	"os"
	"strings"
)

func main() {
	input := string(readInput())
	lines := strings.Split(input, "\n")
	out := make([]rune, len(lines[0]))
	for i := 0; i < len(lines); i++ {
		digit, err := mostCommonAt(lines, i)
		handle(err)

		out[i] = digit
	}

	fmt.Println(out)
}

func readInput() []byte {
	contents, err := ioutil.ReadAll(os.Stdin)
	handle(err)

	return contents
}

func handle(err error) {
	if err != nil {
		panic(err)
	}
}
