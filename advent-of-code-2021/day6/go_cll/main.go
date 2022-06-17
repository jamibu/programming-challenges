package main

import (
	"io/ioutil"
	"strconv"
	"strings"
)

func main() {
	intitialState := read_input("input.txt")

}

type Node struct {
	fish int
	next *Node
}

type CircularLinkedList struct {
	head *Node
	tail *Node
}

func (s *CircularLinkedList) rotateOne struct {
	
}


type SingleLinkedList struct{}

func (s *SingleLinkedList) Push() {}

func (s *SingleLinkedList) Pop() {}

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
