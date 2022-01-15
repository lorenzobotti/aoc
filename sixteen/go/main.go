package main

import (
	"fmt"
	"io/ioutil"
	"os"

	"github.com/k0kubun/pp"
)

func main() {
	input := readIn()
	reader := ParseHex(input)
	packet := reader.ParsePacket()

	pp.Println(packet)
	// fmt.Println(packet)
	fmt.Println(packet.Value())
}

func readIn() string {
	contents, err := ioutil.ReadAll(os.Stdin)
	if err != nil {
		panic(err)
	}

	return string(contents)
}
