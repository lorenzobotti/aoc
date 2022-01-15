package main

import (
	"fmt"
	"io/ioutil"
	"os"
)

func main() {
	input := readIn()
	floor := ParseFloor(input)

	steps := floor.UntilGridLock()
	fmt.Println(steps)
}

func readIn() string {
	contents, err := ioutil.ReadAll(os.Stdin)
	if err != nil {
		panic(err)
	}

	return string(contents)
}
