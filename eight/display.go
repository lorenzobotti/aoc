package main

import "strings"

// 0:      1:      2:      3:      4:
// aaaa    ....    aaaa    aaaa    ....
// b    c  .    c  .    c  .    c  b    c
// b    c  .    c  .    c  .    c  b    c
// ....    ....    dddd    dddd    dddd
// e    f  .    f  e    .  .    f  .    f
// e    f  .    f  e    .  .    f  .    f
// gggg    ....    gggg    gggg    ....

//  5:      6:      7:      8:      9:
// aaaa    aaaa    aaaa    aaaa    aaaa
// b    .  b    .  .    c  b    c  b    c
// b    .  b    .  .    c  b    c  b    c
// dddd    dddd    ....    dddd    dddd
// .    f  e    f  .    f  e    f  .    f
// .    f  e    f  .    f  e    f  .    f
// gggg    gggg    ....    gggg    gggg

var digits = [10]displayed{
	newDisplayed('a', 'b', 'c', 'e', 'f', 'g'),      // 0
	newDisplayed('c', 'f'),                          // 1
	newDisplayed('a', 'c', 'd', 'e', 'g'),           // 2
	newDisplayed('a', 'c', 'd', 'f', 'g'),           // 3
	newDisplayed('b', 'c', 'd', 'f'),                // 4
	newDisplayed('a', 'b', 'd', 'f', 'g'),           // 5
	newDisplayed('a', 'b', 'd', 'e', 'f', 'g'),      // 6
	newDisplayed('a', 'c', 'f'),                     // 7
	newDisplayed('a', 'b', 'c', 'd', 'e', 'f', 'g'), // 8
	newDisplayed('a', 'b', 'c', 'd', 'f', 'g'),      // 9
}

func digitsWithNSegments(segments int) []displayed {
	out := []displayed{}
	for _, digit := range digits {
		if len(digit.segments()) == segments {
			out = append(out, digit)
		}
	}
	return out
}

func (d displayed) similarSegments(other displayed) []rune {
	similar := []rune{}

	if d.a == other.a {
		similar = append(similar, 'a')
	}
	if d.b == other.b {
		similar = append(similar, 'b')
	}
	if d.c == other.c {
		similar = append(similar, 'c')
	}
	if d.d == other.d {
		similar = append(similar, 'd')
	}
	if d.e == other.e {
		similar = append(similar, 'e')
	}
	if d.f == other.f {
		similar = append(similar, 'f')
	}
	if d.g == other.g {
		similar = append(similar, 'g')
	}

	return similar
}

func (d displayed) union(other displayed) displayed {
	return displayed{
		a: d.a && other.a,
		b: d.b && other.b,
		c: d.c && other.c,
		d: d.d && other.d,
		e: d.e && other.e,
		f: d.f && other.f,
		g: d.g && other.g,
	}
}

func (d displayed) uniqueSegmentCount() bool {
	segments := len(d.segments())
	return len(digitsWithNSegments(segments)) == 1
}

func (d displayed) compactString() string {
	builder := strings.Builder{}

	if d.a {
		builder.WriteRune('a')
	}
	if d.b {
		builder.WriteRune('b')
	}
	if d.c {
		builder.WriteRune('c')
	}
	if d.d {
		builder.WriteRune('d')
	}
	if d.e {
		builder.WriteRune('e')
	}
	if d.f {
		builder.WriteRune('f')
	}
	if d.g {
		builder.WriteRune('g')
	}

	return builder.String()
}

type displayed struct {
	a, b, c, d, e, f, g bool
}

func newDisplayed(digits ...rune) displayed {
	out := displayed{}
	for _, digit := range digits {
		switch digit {
		case 'a':
			out.a = true
		case 'b':
			out.b = true
		case 'c':
			out.c = true
		case 'd':
			out.d = true
		case 'e':
			out.e = true
		case 'f':
			out.f = true
		case 'g':
			out.g = true
		}
	}
	return out
}

func displayedFromString(input string) displayed {
	return newDisplayed([]rune(input)...)
}

func (d displayed) segments() []rune {
	out := []rune{}
	if d.a {
		out = append(out, 'a')
	}
	if d.b {
		out = append(out, 'b')
	}
	if d.c {
		out = append(out, 'c')
	}
	if d.d {
		out = append(out, 'd')
	}
	if d.e {
		out = append(out, 'e')
	}
	if d.f {
		out = append(out, 'f')
	}
	if d.g {
		out = append(out, 'g')
	}
	return out
}

func (d displayed) String() string {
	builder := strings.Builder{}
	if d.a {
		builder.WriteString(" aaaa \n")
	} else {
		builder.WriteString("      \n")
	}

	for i := 0; i < 4; i++ {
		if d.b {
			builder.WriteRune('b')
		} else {
			builder.WriteRune(' ')
		}

		builder.WriteString("    ")

		if d.c {
			builder.WriteRune('c')
		} else {
			builder.WriteRune(' ')
		}

		builder.WriteRune('\n')

	}

	if d.d {
		builder.WriteString(" dddd \n")
	} else {
		builder.WriteString("      \n")
	}

	for i := 0; i < 4; i++ {
		if d.e {
			builder.WriteRune('e')
		} else {
			builder.WriteRune(' ')
		}

		builder.WriteString("    ")

		if d.f {
			builder.WriteRune('f')
		} else {
			builder.WriteRune(' ')
		}

		builder.WriteRune('\n')
	}

	if d.g {
		builder.WriteString(" gggg \n")
	} else {
		builder.WriteString("      \n")
	}

	return builder.String()
}
