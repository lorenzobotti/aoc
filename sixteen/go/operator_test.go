package main

import "testing"

func TestValues(t *testing.T) {
	testCases := []struct {
		desc     string
		input    string
		solution int
	}{
		{
			desc:     "one",
			input:    "C200B40A82",
			solution: 3,
		},
		{
			desc:     "two",
			input:    "04005AC33890",
			solution: 54,
		},
		{
			desc:     "three",
			input:    "880086C3E88112",
			solution: 7,
		},
		{
			desc:     "four",
			input:    "CE00C43D881120",
			solution: 9,
		},
		{
			desc:     "five",
			input:    "D8005AC2A8F0",
			solution: 1,
		},
		{
			desc:     "six",
			input:    "F600BC2D8F",
			solution: 0,
		},
		{
			desc:     "seven",
			input:    "9C005AC2F8F0",
			solution: 0,
		},
		{
			desc:     "eight",
			input:    "9C0141080250320F1802104A08",
			solution: 1,
		},
	}
	for _, tc := range testCases {
		t.Run(tc.desc, func(t *testing.T) {
			reader := ParseHex(tc.input)
			packet := reader.ParsePacket()

			if packet.Value() != tc.solution {
				t.Fail()
			}
		})
	}
}
