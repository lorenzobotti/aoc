package main

import (
	"fmt"
	"io/ioutil"
	"os"
)

func main() {
	input := readAll()
	subs := newSub(input)

	spot, fuel := subs.mostEfficientSpot()
	fmt.Println("best spot:", spot, "takes", fuel, "fuel")
}

func readAll() string {
	contents, err := ioutil.ReadAll(os.Stdin)
	if err != nil {
		panic(err)
	}

	return string(contents)
}
