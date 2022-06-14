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

	for boardNum, boardString := range boardStrings {
		game.AddBoard(boardString, boardNum)
	}

	for _, num := range nums {
		game.DrawNumber(num)
	}

	bestBoardNum := &game.finished[0]
	worstBoardNum := &game.finished[len(game.finished)-1]

	fmt.Println("Best board: ")
	winningBoard := game.boards[*bestBoardNum]
	winningBoard.display()

	fmt.Println("The winning number was ", winningBoard.lastNumber)
	fmt.Println("Remaining numbers add to: ", winningBoard.remainingSum)

	answer1 := (winningBoard.remainingSum * winningBoard.lastNumber)
	fmt.Println("Answer = ", answer1)

	fmt.Println("Best board: ")
	losingBoard := game.boards[*worstBoardNum]
	losingBoard.display()

	fmt.Println("The winning number was ", losingBoard.lastNumber)
	fmt.Println("Remaining numbers add to: ", losingBoard.remainingSum)

	answer2 := (losingBoard.remainingSum * losingBoard.lastNumber)
	fmt.Println("Answer = ", answer2)

}

func ReadInput(fname string) ([]int, []string) {
	content, _ := ioutil.ReadFile(fname)

	text := string(content)

	splitStrings := strings.Split(text, "\n\n")
	numberLine := splitStrings[0]
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
	board        [5][5]int
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

	for num := 0; num <= 99; num++ {
		numCoords[num] = []NumberCoord{}
	}

	return BingoGame{boards: boards, numCoords: numCoords, finished: finished}
}

func (g *BingoGame) AddBoard(boardString string, boardNum int) {
	var boardArray [5][5]int
	var boardSum int
	rows := strings.Split(boardString, "\n")

	for r, rowString := range rows {
		numberStrings := strings.Fields(rowString)

		for c, numString := range numberStrings {
			num, _ := strconv.Atoi(numString)
			boardArray[r][c] = num
			boardSum += num

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
	for _, coord := range g.numCoords[num] {
		if g.boards[coord.board].hasWon {
			continue
		}

		winner = g.boards[coord.board].MarkNumber(coord.row, coord.col, num)

		if winner {
			g.finished = append(g.finished, coord.board)
		}
	}
}

func (b *BingoBoard) MarkNumber(row int, col int, num int) bool {
	b.board[row][col] = -1
	b.remainingSum -= num
	b.rowScore[row] += 1
	b.colScore[col] += 1

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
