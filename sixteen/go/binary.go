package main

import (
	"encoding/hex"
	"strings"
)

type BoolReader struct {
	content []bool
}

func (b BoolReader) Peek(n int) []bool {
	return b.content[:n]
}

func (b *BoolReader) Consume(n int) []bool {
	out := b.content[:n]

	b.content = b.content[n:]
	return out
}

func (b BoolReader) EOF() bool {
	return len(b.content) == 0
}

func (b BoolReader) BitsLeft() int {
	return len(b.content)
}

func newBoolReader(bits []bool) BoolReader {
	return BoolReader{
		content: bits,
	}
}

func toNumber(bytes []bool) int {
	out := 0

	for _, b := range bytes {
		out = out << 1
		if b {
			out += 1
		}
	}

	return out
}

func ParseHex(input string) BoolReader {
	bytes, err := hex.DecodeString(strings.TrimSpace(input))
	if err != nil {
		panic(err)
	}

	bools := []bool{}

	for _, byt := range bytes {
		for i := 7; i >= 0; i-- {
			mask := byte(1) << i
			onlyBit := byt & mask

			bools = append(bools, onlyBit != 0)
		}
	}

	return BoolReader{bools}
}
