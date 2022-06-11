package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

func main() {
	f, _ := os.Open("./input.txt")
	scanner := bufio.NewScanner(f)

	var numOnes [12]int
	var count int

	// Each line contains a single binary string
	for scanner.Scan() {
		line := scanner.Text()

		// Need to get each bit from each string
		countOnes(line, &numOnes)
	}

	// Array telling us whether 1 is the most common bit
	gamma, epsilon := calcGammaAndEpsilon(&numOnes, &count)

	fmt.Println("Answer = ", gamma*epsilon)
}

func countOnes(line string, numOnes *[12]int) {
	for i, r := range line {
		bit := int(r - '0')
		numOnes[i] += bit
	}
}

func calcGammaAndEpsilon(numOnes *[12]int, numTotal *int) (int64, int64) {
	var gammaString string
	var epsilonString string

	halfTotal := *numTotal / 2

	for _, num := range *numOnes {
		if num > halfTotal {
			gammaString += "1"
			epsilonString += "0"
		} else {
			gammaString += "0"
			epsilonString += "1"
		}
	}

	gamma, _ := strconv.ParseInt(gammaString, 2, 64)
	epsilon, _ := strconv.ParseInt(epsilonString, 2, 64)

	return gamma, epsilon
}
