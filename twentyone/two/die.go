package main

type Die int

func (d *Die) Roll() int {
	val := *d

	if *d == 100 {
		*d = 0
	}

	*d += 1
	return int(val)
}

func (d *Die) RollN(n int) []int {
	out := []int{}

	for i := 0; i < n; i++ {
		out = append(out, d.Roll())
	}

	return out
}

func newDie() Die {
	return 1
}
