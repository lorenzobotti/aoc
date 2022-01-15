package main

import (
	"io/ioutil"
	"testing"
)

func TestVersion(t *testing.T) {
	testCases := []struct {
		desc     string
		filename string
		solution int
	}{
		{
			desc:     "sample 4",
			filename: "samples/sample_4.txt",
			solution: 16,
		},
		{
			desc:     "sample 5",
			filename: "samples/sample_5.txt",
			solution: 12,
		},
		{
			desc:     "sample 6",
			filename: "samples/sample_6.txt",
			solution: 23,
		},
		{
			desc:     "sample 7",
			filename: "samples/sample_7.txt",
			solution: 31,
		},
	}
	for _, tc := range testCases {
		t.Run(tc.desc, func(t *testing.T) {
			contents, err := ioutil.ReadFile(tc.filename)
			if err != nil {
				t.Fail()
			}

			parser := ParseHex(string(contents))
			packet := parser.ParsePacket()

			if packet.VersionSum() != tc.solution {
				t.Fail()
			}
		})
	}
}
