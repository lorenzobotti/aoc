package main

import "strings"

func max(a, b int) int {
	if a > b {
		return a
	} else {
		return b
	}
}

func min(a, b int) int {
	if a < b {
		return a
	} else {
		return b
	}
}

func splitInTwo(input, separator string) (string, string) {
	parts := strings.SplitN(input, separator, 2)
	if len(parts) == 1 {
		return parts[0], ""
	} else {
		return parts[0], parts[1]
	}
}

func mergeLines(a, b string) string {
	linesA := strings.Split(a, "\n")
	linesB := strings.Split(b, "\n")

	mostLines := linesA
	if len(b) > len(a) {
		mostLines = linesB
	}

	minLength := min(len(a), len(b))
	mostLength := max(len(a), len(b))

	out := strings.Builder{}
	for i := 0; i < minLength; i++ {
		out.WriteString(linesA[i])
		out.WriteString("\t")
		out.WriteString(linesB[i])
		out.WriteRune('\n')
	}

	if len(mostLines) <= minLength {
		return out.String()
	}

	for i := minLength; i < mostLength; i++ {
		out.WriteString(mostLines[i])
		out.WriteRune('\n')
	}

	return out.String()
}
