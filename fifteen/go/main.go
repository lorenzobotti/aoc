package main

import (
	"fmt"
	"io/ioutil"
	"os"
)

func main() {
	input := readAllInput()
	fmt.Println(input)
}

func readAllInput() string {
	contents, err := ioutil.ReadAll(os.Stdin)
	if err != nil {
		panic(err)
	}

	return string(contents)
}
