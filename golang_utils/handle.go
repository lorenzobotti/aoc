package utils

import (
	"fmt"
	"os"
)

func Handle(err interface{}) {
	if err != nil {
		fmt.Fprintln(os.Stderr, err)
		os.Exit(1)
	}
}
