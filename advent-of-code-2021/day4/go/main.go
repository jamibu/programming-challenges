package main

import (
	"fmt"
	"strconv"
	"strings"
)

func main() {

	input := []string{
		"37 72 60 35 89",
		"32 49  4 77 82",
		"30 26 27 63 88",
		"29 43 16 34 58",
		"48 33 96 79 94",
		"/n",
		"41 94 77 43 87",
		"2 17 82 96 25",
		"95 49 32 12  9",
		"59 33 67 71 64",
		"88 54 93 85 30",
	}

	boardNum := 0
	board := BingoBoard{}
	var boards []BingoBoard
	var rowNum int
	numbers := make(map[int][]Number)

	for _, line := range input {

		if line == "/n" {
			rowNum = 0
			boardNum += 1
			boards = append(boards, board)
			board = NewBoard()
		}

		for c, n := range strings.Fields(line) {
			fmt.Println(c)
			fmt.Println(rowNum)
			num, _ := strconv.Atoi(n)
			board.addNumber(rowNum, c, num)

			if _, ok := numbers[num]; !ok {
				numbers[num] = []Number{}
			}

			numbers[num] = append(numbers[num], Number{num, boardNum, rowNum, c})
		}
		rowNum++
	}

	boards[0].display()
}

type Number struct {
	value int
	board int
	row   int
	col   int
}

// Individual board
type BingoBoard struct {
	board  [5][5]int
	rowSum [5]int
	colSum [5]int
}

func NewBoard() BingoBoard {
	return BingoBoard{
		board:  [5][5]int{},
		rowSum: [5]int{},
		colSum: [5]int{},
	}
}

// Add number to a postion on the board
func (b *BingoBoard) addNumber(row int, col int, number int) {
	b.board[row][col] = number
	b.rowSum[row] += number
	b.colSum[row] += number
}

// Mark number as being called
func (b *BingoBoard) markNumber(row int, col int, number int) {
	b.board[row][col] = -1
	b.rowSum[row] -= number + 1
	b.colSum[col] -= number + 1
}

// Display a board
func (b BingoBoard) display() {
	for _, row := range b.board {
		fmt.Println(row)
	}
}
