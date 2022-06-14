package main

import (
	"fmt"
	"io/ioutil"
	"strconv"
	"strings"
)

func main() {
	game := NewGame(0, 99)

	nums, boardStrings := ReadInput("inputs.txt")

	// Parse boards into game
	for boardNum, boardString := range boardStrings {
		game.AddBoard(boardString, boardNum)
	}

	// Draw all number
	for _, num := range nums {
		game.DrawNumber(num)
	}

	// Find first and last boards to finish
	bestBoardNum := &game.finished[0]
	worstBoardNum := &game.finished[len(game.finished)-1]

	// Show results
	fmt.Println("===== Best board =====")
	PrintResults(game.boards[*bestBoardNum])
	fmt.Println("")
	fmt.Println("===== Worst board ====")
	PrintResults(game.boards[*worstBoardNum])
}

func ReadInput(fname string) ([]int, []string) {
	content, _ := ioutil.ReadFile(fname)
	text := string(content)

	// Each input is separated by a blank line
	splitStrings := strings.Split(text, "\n\n")
	// First line is different, contains the numbers that will be read out
	numberLine := splitStrings[0]
	// The rest is the bingo boards. Will leave these as blocks of string
	boards := splitStrings[1:]

	numStrings := strings.Split(numberLine, ",")
	var nums []int
	for _, n := range numStrings {
		num, _ := strconv.Atoi(n)
		nums = append(nums, num)
	}

	return nums, boards
}

type NumberCoord struct {
	board int
	row   int
	col   int
}

type BingoBoard struct {
	board        [5][5]string
	remainingSum int
	rowScore     [5]int
	colScore     [5]int
	hasWon       bool
	lastNumber   int
}

type BingoGame struct {
	boards    []*BingoBoard
	numCoords map[int][]NumberCoord
	finished  []int
}

func NewGame(start int, end int) BingoGame {
	boards := []*BingoBoard{}
	numCoords := map[int][]NumberCoord{}
	finished := []int{}

	// Predefine map we an empty slice for each number
	// Will fill this with the boards that contain the number
	for num := 0; num <= 99; num++ {
		numCoords[num] = []NumberCoord{}
	}

	return BingoGame{boards: boards, numCoords: numCoords, finished: finished}
}

func (g *BingoGame) AddBoard(boardString string, boardNum int) {
	var boardArray [5][5]string
	var boardSum int

	rows := strings.Split(boardString, "\n")
	for r, rowString := range rows {
		numberStrings := strings.Fields(rowString)

		for c, numString := range numberStrings {
			num, _ := strconv.Atoi(numString)

			// Padding single didget numbers with space so our board displays properly
			if len(numString) == 1 {
				numString = " " + numString
			}
			boardArray[r][c] = numString
			boardSum += num

			// Storing everyboard that contains a number and the row and col it is on
			// Saves us iterating over every board each time
			g.numCoords[num] = append(g.numCoords[num], NumberCoord{boardNum, r, c})
		}
	}

	board := &BingoBoard{
		board:        boardArray,
		remainingSum: boardSum,
		rowScore:     [...]int{0, 0, 0, 0, 0},
		colScore:     [...]int{0, 0, 0, 0, 0},
	}

	g.boards = append(g.boards, board)
}

func (g *BingoGame) DrawNumber(num int) {
	winner := false
	// Iterates over every board that has a number
	for _, coord := range g.numCoords[num] {
		// Don't want to keep playing on boards that have won
		if g.boards[coord.board].hasWon {
			continue
		}

		// Mark number on this particular board
		winner = g.boards[coord.board].MarkNumber(coord.row, coord.col, num)

		// Store boards that have won so we can see order of winners later
		if winner {
			g.finished = append(g.finished, coord.board)
		}
	}
}

func (b *BingoBoard) MarkNumber(row int, col int, num int) bool {
	// Visual mark on the board
	b.board[row][col] = " X"
	// Remove marked number from the sum of the numbers so we can tell the sum of
	// what is left over
	b.remainingSum -= num
	// Keep score of how many are marked in each row and column so we know if the board has one
	b.rowScore[row] += 1
	b.colScore[col] += 1

	// See if we have won
	for i := 0; i < 5; i++ {
		if b.rowScore[i] == 5 || b.colScore[i] == 5 {
			b.hasWon = true
			b.lastNumber = num
			break
		}
	}

	return b.hasWon
}

func (b BingoBoard) display() {
	for _, row := range b.board {
		fmt.Println(row)
	}
}

func PrintResults(board *BingoBoard) {
	board.display()
	fmt.Println("The winning number was:", board.lastNumber)
	fmt.Println("Remaining numbers add to:", board.remainingSum)
	answer := (board.remainingSum * board.lastNumber)
	fmt.Println("Answer =", answer)
}
