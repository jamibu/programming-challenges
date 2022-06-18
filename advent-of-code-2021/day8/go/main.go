package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strings"
)

func main() {
	_, outPatterns := readInput("input.txt")

	var count1 int
	var count4 int
	var count7 int
	var count8 int
	for _, pattern := range outPatterns {
		for _, p := range pattern {
			switch len(p) {
			case 2:
				count1++
			case 3:
				count7++
			case 4:
				count4++
			case 7:
				count8++
			}
		}
	}

	fmt.Println("Part 1:", count1+count7+count4+count8)
}

func readInput(fname string) ([][]string, [][]string) {
	inPatterns := [][]string{}
	outPatterns := [][]string{}

	file, err := os.Open("input.txt")
	if err != nil {
		log.Fatal(err)
	}

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		patterns := strings.Fields(scanner.Text())
		inPatterns = append(inPatterns, patterns[0:10])
		outPatterns = append(outPatterns, patterns[11:])
	}

	return inPatterns, outPatterns
}
