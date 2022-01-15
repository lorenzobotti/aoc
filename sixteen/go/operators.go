package main

func (p Packet) Value() int {
	if p.IsLiteral() {
		return p.literalValue
	}

	values := []int{}
	for _, child := range p.subPackets {
		values = append(values, child.Value())
	}

	return operators[p.typeID](values...)
}

var operators = map[byte]func(...int) int{
	0: sum,
	1: product,
	2: min,
	3: max,
	5: greater,
	6: lesser,
	7: equal,
}

func sum(numbers ...int) int {
	out := 0

	for _, num := range numbers {
		out += num
	}

	return out
}

func product(numbers ...int) int {
	out := numbers[0]

	for _, num := range numbers[1:] {
		out *= num
	}

	return out
}

func max(numbers ...int) int {
	biggest := numbers[0]

	for _, num := range numbers {
		if num > biggest {
			biggest = num
		}
	}

	return biggest
}

func min(numbers ...int) int {
	smallest := numbers[0]

	for _, num := range numbers {
		if num < smallest {
			smallest = num
		}
	}

	return smallest
}

func greater(numbers ...int) int {
	if numbers[0] > numbers[1] {
		return 1
	} else {
		return 0
	}
}

func lesser(numbers ...int) int {
	if numbers[0] < numbers[1] {
		return 1
	} else {
		return 0
	}
}

func equal(numbers ...int) int {
	if numbers[0] == numbers[1] {
		return 1
	} else {
		return 0
	}
}
