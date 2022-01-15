package main

import (
	_ "embed"
	"fmt"
	"io/ioutil"
	"log"
	"os"

	"github.com/k0kubun/pp"
)

// //go:embed sample.txt
// var input string

func main() {
	log.Println("getting input")
	input, err := parseIn()
	if err != nil {
		panic(err)
	}

	log.Println("parsing")

	players, err := parsePlayers(input)
	if err != nil {
		panic(err)
	}
	pp.Println(players)

	log.Println("playing")
	fmt.Println(Play(players))
}

func parseIn() (string, error) {
	content, err := ioutil.ReadAll(os.Stdin)
	return string(content), err
}
