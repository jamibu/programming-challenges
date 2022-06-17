package main

import (
	"fmt"
	"io/ioutil"
	"strconv"
	"strings"
)

func main() {
	// Initial state of fish population
	intitialState := read_input("input.txt")

	timer1 := NewFishTimer(intitialState)
	timer1.cycleDays(80)

	fmt.Println("Total Fish after 80 days", timer1.countFish())

	timer2 := NewFishTimer(intitialState)
	timer2.cycleDays(256)

	fmt.Println("Total Fish after 256 days", timer2.countFish())
}

type FishTimer struct {
	newFish []int
	oldFish []int
}

func NewFishTimer(initialState []int) FishTimer {
	// Number of fish for each stage in cycle (i.e. 2 is 2 days left)
	currentPopulation := []int{0, 0, 0, 0, 0, 0, 0}

	// Initial State contain stage of cycle for every fish
	for _, stage := range initialState {
		currentPopulation[stage] += 1
	}

	// Initial state does not containe new fish
	return FishTimer{newFish: []int{0, 0}, oldFish: currentPopulation}
}

func (t *FishTimer) cycleDays(numDays int) {
	// Each iteration is the passing of a day
	for i := 0; i < numDays; i++ {
		day0Fish := t.oldFish[0]
		day7Fish := t.newFish[0]

		// New fish on 7 -> 6 become old fish Old fish on 0 reset to 6
		t.oldFish = append(t.oldFish[1:], day0Fish+day7Fish)

		// Fish on  0 spawn an equal number of fish which are at 8
		t.newFish = append(t.newFish[1:], day0Fish)
	}
}

func (t FishTimer) countFish() int {
	totalFish := 0
	for _, fish := range t.oldFish {
		totalFish += fish
	}

	for _, fish := range t.newFish {
		totalFish += fish
	}

	return totalFish
}

func read_input(fname string) []int {
	content, _ := ioutil.ReadFile(fname)
	fishStrings := strings.Split(string(content), ",")

	initialState := []int{}
	for _, fish := range fishStrings {
		fish, _ := strconv.Atoi(fish)
		initialState = append(initialState, fish)
	}

	return initialState
}
