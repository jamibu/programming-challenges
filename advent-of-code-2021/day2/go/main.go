package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func main() {
	// Movement direction and magnitude stored in text file
	movements, _ := readDirections("./input.txt")

	// Tracks position where up or down directly changes the depth
	pos := position{0, 0, 0}
	// Tracks position where up or down changes the aim
	posWithAim := position{0, 0, 0}

	// Apply all movements to find new position
	for _, mov := range movements {
		pos.move(mov)
		posWithAim.moveWithAim(mov)
	}

	fmt.Println("Result - Part 1")
	fmt.Println(pos.horizontal * pos.vertical)
	fmt.Println("Result - Part 2")
	fmt.Println(posWithAim.horizontal * posWithAim.vertical)
}

type movement struct {
	direction string
	magnitude int
}

type position struct {
	horizontal int
	vertical   int
	aim        int
}

func (p *position) move(m movement) {
	// Apply movement to postion to compute new position
	switch m.direction {
	case "forward":
		p.horizontal += m.magnitude
	case "down":
		p.vertical += m.magnitude
	case "up":
		p.vertical -= m.magnitude
	}
}

func (p *position) moveWithAim(m movement) {
	// Apply movement to postion to compute new position or aim
	switch m.direction {
	case "forward":
		p.horizontal += m.magnitude
		p.vertical += p.aim * m.magnitude
	case "down":
		p.aim += m.magnitude
	case "up":
		p.aim -= m.magnitude
	}
}

func readDirections(fname string) ([]movement, error) {
	f, _ := os.Open(fname)
	scanner := bufio.NewScanner(f)

	// Will return list of movements
	var movements []movement

	// Each line contains a movement
	for scanner.Scan() {
		line := scanner.Text()
		// Each line has direction as a string and the magnitude of the movement
		directions := strings.Fields(line)
		magnitude, _ := strconv.Atoi(directions[1])

		// Create movement
		m := movement{directions[0], magnitude}
		movements = append(movements, m)
	}

	return movements, scanner.Err()
}
