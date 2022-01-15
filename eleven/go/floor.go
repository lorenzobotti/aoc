package main

type seafloor struct {
	octopuses map[coord]octopus
	rows      int
	columns   int
}

func newSeafloor(rows, columns int) seafloor {
	return seafloor{
		octopuses: map[coord]octopus{},
		rows:      rows,
		columns:   columns,
	}
}

func (s seafloor) neighbours(c coord) []coord {

}
