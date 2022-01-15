package main

import (
	"testing"
)

func TestUnique(t *testing.T) {
	for i := 0; i < len(digits); i++ {
		unique := digits[i].uniqueSegmentCount()
		shouldBeUnique := i == 1 || i == 4 || i == 7 || i == 8

		if unique != shouldBeUnique {
			t.Fail()
		}
	}

}
