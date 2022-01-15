package main

type distanceField struct {
	field cave

	distances map[coord]int
}

func newDistanceField(field cave) distanceField {
	return distanceField{
		field:     field,
		distances: map[coord]int{},
	}
}

func (d distanceField) solve() {

}
