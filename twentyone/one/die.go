package main

type Die struct {
	val         int
	timesRolled int
}

func NewDie() Die {
	return Die{
		val:         1,
		timesRolled: 0,
	}
}

func (d *Die) Roll() int {
	if d.val > 100 {
		d.val = 1
	}

	roll := d.val
	d.val += 1
	d.timesRolled += 1

	return roll
}

func (d *Die) RollN(n int) []int {
	out := []int{}

	for i := 0; i < n; i++ {
		out = append(out, d.Roll())
	}

	return out
}
