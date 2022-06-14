package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func main() {
	x1Coords, y1Coords, x2Coords, y2Coords := ReadCoordinates("inputs.txt")
	// x1Coords, y1Coords, x2Coords, y2Coords := ReadCoordinates("inputs.txt")

	maxX := FindMax(append(x1Coords, x2Coords...))
	maxY := FindMax(append(y1Coords, y2Coords...))

	matrix := NewMatrix(maxX, maxY)

	for i := 0; i < len(x1Coords); i++ {
		line := NewLine(x1Coords[i], y1Coords[i], x2Coords[i], y2Coords[i])
		matrix.AddLine(line)
	}

	crosses := matrix.CountIntersects()
	matrix.display()

	fmt.Println(crosses)

}

func FindMax(slice []int) int {
	max := slice[0]
	for _, val := range slice[1 : len(slice)-1] {
		if val > max {
			max = val
		}
	}

	return max
}

func ReadCoordinates(fname string) ([]int, []int, []int, []int) {
	file, _ := os.Open(fname)
	scanner := bufio.NewScanner(file)

	var x1Coords []int
	var y1Coords []int
	var x2Coords []int
	var y2Coords []int

	for scanner.Scan() {
		line := scanner.Text()
		points := strings.Split(line, " -> ")

		point1 := strings.Split(points[0], ",")
		point2 := strings.Split(points[1], ",")

		x1, _ := strconv.Atoi(point1[0])
		y1, _ := strconv.Atoi(point1[1])
		x2, _ := strconv.Atoi(point2[0])
		y2, _ := strconv.Atoi(point2[1])

		x1Coords = append(x1Coords, x1)
		y1Coords = append(y1Coords, y1)
		x2Coords = append(x2Coords, x2)
		y2Coords = append(y2Coords, y2)
	}

	return x1Coords, y1Coords, x2Coords, y2Coords
}

type Line struct {
	x1 int
	y1 int
	x2 int
	y2 int
	m int
	c int
}

func NewLine(x1, y1, x2, y2 int) Line {
	xDiff := x2 - x1

	var m int
	var c int

	if xDiff == 0 {
		m = 0
		c = 0
	} else {
		m = (y2 - y1) / (x2 - x1)
		c = y1 - m*x1
	}

	return Line{x1, y1, x2, y2, m, c}
}

func (l Line) CalcY(x int) int {
	return x*l.m + l.c
}

func (l Line) AsArrays() ([]int, []int) {
	xArray := []int{}
	yArray := []int{}

	if l.x1 == l.x2 {
		minY, maxY := MinMax(l.y1, l.y2)

		for y := minY; y <= maxY; y++ {
			xArray = append(xArray, l.x1)
			yArray = append(yArray, y)	
		}
	} else {
		minX, maxX := MinMax(l.x1, l.x2)

		for x := minX; x <= maxX; x++ {
			xArray = append(xArray, x)
			yArray = append(yArray, l.CalcY(x))
		}
	}

	return xArray, yArray
}

func MinMax(i1 int, i2 int) (int, int) {
	var min int
	var max int
	if i1 > i2 {
		min = i2
		max = i1
	} else {
		min = i1
		max = i2
	}

	return min, max
}

type Matrix struct {
	matrix [][]int
}

func NewMatrix(xMax int, yMax int) Matrix {
	matrix := [][]int{}
	for y := 0; y <= yMax; y++ {
		row := []int{}
		for x := 0; x <= xMax; x++ {
			row = append(row, 0)
		}
		matrix = append(matrix, row)
	}

	return Matrix{matrix}
}

func (m *Matrix) AddLine(line Line) {
	xArray, yArray := line.AsArrays()

	for i := 0; i < len(xArray); i++ {
		x := xArray[i]
		y := yArray[i]
		m.matrix[y][x] += 1
	}
}

func (m Matrix) CountIntersects() int {
	width := len(m.matrix[0])
	count := 0
	for y := 0; y < len(m.matrix); y++ {
		for x := 0; x < width; x++ {
			if m.matrix[y][x] > 1 {
				count++
			}
		}
	}

	return count
}

func (m Matrix) display() {
	for _, row := range m.matrix {
		fmt.Println(row)
	}
}