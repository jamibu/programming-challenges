package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strings"
)

func main() {
	inPatterns, outPatterns := readInput("input.txt")

	part1 := solvePart1(outPatterns)

	fmt.Println("Part 1:", part1)
}

func solvePart1(outPatterns [][]string) int {
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

	return count1 + count7 + count4 + count8
}

type Decoder struct {
	numbers  map[int]string
	encoding map[rune]string
}

func (d *Decoder) findNumbers(inputs []string) {
	unknown := make([]string)
	for _, p := range inputs {
		switch len(p) {
		case 2:
			d.numbers[1] = p
		case 3:
			d.numbers[7] = p
		case 4:
			d.numbers[4] = p
		case 7:
			d.numbers[8] = p
		default:
			append(unknown, p)
		}
	}

	unknown2 := make([]string)
	for _, p := range unknown {
		p_len := len(p)
		if p_len == 5 && numOverlap(d.numbers[1], p) == 2 {
			d.numbers[3] == p
		} else if p_len == 6 && numOverlap(d.numbers[7], p) == 2 {
			d.numbers[6] == p
		} else if p_len == 6 && numOverlap(d.numbers[4], p) == 4 {
			d.numbers[9] = p
		}
	}

	for _, p := range unknown2 {
		switch len(p) {
		case 5:
			d.numbers[2] = p
		case 6:
			d.numbers[0] = p
		}
	}
}

func (d Decoder) decode() {
	fmt.Println(1)
}

func numOverlap(code string, pattern string) {
	count := 0
	for _, char := range code {
		if strings.ContainsRune(code, char) {
			count++
		}
	}
	return count
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
