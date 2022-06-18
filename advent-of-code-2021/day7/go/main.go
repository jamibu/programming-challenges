package main

import (
	"fmt"
	"io/ioutil"
	"log"
	"math"
	"sort"
	"strconv"
	"strings"
)

func main() {
	// Read inputs, each point is a crab with the position of
	crabPositions := readInputs("input.txt")

	// Part 1
	bestPos1 := medianPos((crabPositions))
	fuel1 := calcFuel(crabPositions, bestPos1)

	fmt.Printf(
		"Part 1: Crabs should go to %d, this will use %d fuel\n",
		bestPos1, int(fuel1),
	)

	// Part 2
	bestPos2 := meanPos(crabPositions)
	bestPos2RoundDown := int(math.Floor(bestPos2))
	bestPos2RoundUp := int(math.Ceil(bestPos2))

	fuel2RoundDown := calcFuel2(crabPositions, bestPos2RoundDown)
	fuel2RoundUp := calcFuel2(crabPositions, bestPos2RoundUp)

	var fuel2 int
	var pos2 int

	if fuel2RoundDown > fuel2RoundUp {
		fuel2 = int(fuel2RoundUp)
		pos2 = bestPos2RoundUp
	} else {
		fuel2 = int(fuel2RoundDown)
		pos2 = bestPos2RoundDown
	}

	fmt.Printf(
		"Part 2: Crabs should go to %d, this will use %d fuel\n",
		pos2, fuel2,
	)
}

func medianPos(crabs []int) int {
	numCrabs := len(crabs)
	sort.Ints(crabs)

	var medianPos int

	if numCrabs == 0 {
		medianPos = 0
	} else if numCrabs%2 == 0 {
		medianPos = (crabs[numCrabs/2-1] + crabs[numCrabs/2]) / 2
	} else {
		medianPos = crabs[numCrabs/2]
	}

	return medianPos
}

func meanPos(crabs []int) float64 {
	var sum int
	numCrabs := len(crabs)
	for _, crab := range crabs {
		sum += crab
	}
	return float64(sum) / float64(numCrabs)
}

func calcFuel(crabs []int, destination int) float64 {
	var fuel float64
	for _, pos := range crabs {
		fuel += math.Abs(float64(destination - pos))
	}

	return fuel
}

func calcFuel2(crabs []int, destination int) float64 {
	var fuel float64
	for _, pos := range crabs {
		dist := math.Abs(float64(destination - pos))
		fuel += dist * (dist + 1) / 2
	}

	return fuel
}

func readInputs(fname string) []int {
	content, errF := ioutil.ReadFile(fname)

	if errF != nil {
		log.Fatal(errF)
	}

	contentStr := string(content)

	crabPositions := []int{}
	for _, crabPosStr := range strings.Split(contentStr, ",") {
		crabPosInt, errI := strconv.Atoi(crabPosStr)

		if errI != nil {
			log.Fatal(errI)
		}

		crabPositions = append(crabPositions, crabPosInt)
	}
	return crabPositions
}
