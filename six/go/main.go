package main

import (
	"flag"
	"fmt"
	"io/ioutil"
	"os"
)

func main() {
	input := readAll()
	// input := readFile()
	// out := bufio.NewWriter(os.Stdout)
	// out := os.Stdout

	days := flag.Int("days", 0, "days to simulate")
	flag.Parse()

	sea, err := ParseBetterSea(input)
	if err != nil {
		panic(err)
	}

	for i := 0; i < *days; i++ {
		// fmt.Fprintln(out, "tick:", i)
		sea.Tick()
		// fmt.Fprintln(out, "ticked:", i)
	}

	fmt.Println(sea.Population())
}

func readAll() string {
	content, err := ioutil.ReadAll(os.Stdin)
	if err != nil {
		panic(err)
	}

	return string(content)
}

func readFile() string {
	content, err := ioutil.ReadFile("../sample.txt")
	if err != nil {
		panic(err)
	}

	return string(content)

}
