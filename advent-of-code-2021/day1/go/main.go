package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

func main() {
	// File contains depth readings
	depths, _ := readDepths("./inputs.txt")

	// Determine if depth changes
	numIncrease := findDepthChange(depths)

	fmt.Println("Part 1")
	fmt.Println(numIncrease)

	// Determine if deth changes within a sliding window of 3
	numIncreaseSliding := windowedDepthChange(depths)

	fmt.Println("Part 2")
	fmt.Println(numIncreaseSliding)

}

func findDepthChange(depths []int) int {
	// Will track depth changes by adding 1 when depth increases
	var numIncrease int
	for i := 0; i < len(depths)-1; i++ {
		if depths[i+1] > depths[i] {
			numIncrease++
		}
	}

	return numIncrease
}

func windowedDepthChange(depths []int) int {
	// Will track depth changes by adding 1 when windowed depth increases
	var numIncrease int
	for i := 0; i < len(depths)-3; i++ {
		// Want to compare one window with the next
		windowSum1 := depths[i] + depths[i+1] + depths[i+2]
		windowSum2 := depths[i+1] + depths[i+2] + depths[i+3]

		if windowSum2 > windowSum1 {
			numIncrease++
		}
	}

	return numIncrease
}

func readDepths(fname string) ([]int, error) {
	f, _ := os.Open(fname)
	scanner := bufio.NewScanner(f)

	// Will return slice of integers
	var result []int

	// Single depth on each line
	for scanner.Scan() {
		depth, _ := strconv.Atoi(scanner.Text())
		result = append(result, depth)
	}

	return result, scanner.Err()
}
