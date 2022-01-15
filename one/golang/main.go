package main

import (
	"fmt"
	"io/ioutil"
	"os"
	"strconv"
	"strings"
)

func main() {
	input := string(readAllStdin())
	lines := strings.Split(input, "\n")
	nums := []int{}
	for _, line := range lines {
		num, err := strconv.Atoi(line)
		handle(err)

		nums = append(nums, num)
	}

	rollingAverage := []int{}
	for i := 1; i < len(nums)-1; i++ {
		sum := nums[i-1] + nums[i] + nums[i+1]
		// avg := sum / 3
		rollingAverage = append(rollingAverage, sum)
	}

	higher := 0
	for i := 1; i < len(rollingAverage); i++ {
		current := rollingAverage[i]
		previous := rollingAverage[i-1]
		if current > previous {
			higher++
		}
	}

	fmt.Println(higher)
}

func readAllStdin() []byte {
	content, err := ioutil.ReadAll(os.Stdin)
	handle(err)

	return content
}

func openAndUnwrap(filename string) []byte {
	file, err := os.Open(filename)
	handle(err)

	res, err := ioutil.ReadAll(file)
	handle(err)

	return res
}

func handle(err interface{}) {
	if err != nil {
		fmt.Fprintln(os.Stderr, err)
		os.Exit(1)
	}
}
