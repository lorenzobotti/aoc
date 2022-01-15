package utils

import "strconv"

func ParseInt(in string) int {
	res, err := strconv.Atoi(in)
	Handle(err)
	return res
}
