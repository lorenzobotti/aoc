package main

import (
	"fmt"
	"io/ioutil"
	"os"
)

var input = `Player 1 starting position: 4
Player 2 starting position: 8`

func main() {
	// input := readIn()

	startingState := NewState(input)
	// firstGen := startingState.Play()

	// secondGenMap := map[GameState]int{}
	// for _, state := range firstGen {
	// 	for _, secondState := range state.Play() {
	// 		secondGenMap[secondState] += 1
	// 	}
	// }

	// pp.Println(secondGenMap)
	one, two := startingState.Universes()
	fmt.Println(one, two)
}

func readIn() string {
	content, err := ioutil.ReadAll(os.Stdin)
	if err != nil {
		panic(err)
	}

	return string(content)
}
