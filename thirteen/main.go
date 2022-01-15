package main

import (
	_ "embed"
	"fmt"
	"io/ioutil"
	"os"
)

//go:embed sample.txt
var inp string

func main() {
	input := readIn()
	sh, folds, err := parse(input)
	// sh, folds, err := parse(inp)
	if err != nil {
		panic(err)
	}

	// fmt.Println(sh)
	for _, f := range folds {
		sh.fold(f)
	}
	fmt.Println(sh)
}

func readIn() string {
	contents, err := ioutil.ReadAll(os.Stdin)
	if err != nil {
		panic(err)
	}

	return string(contents)
}
