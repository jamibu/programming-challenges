package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strings"
    "sort"
    "strconv"
)

func main() {
	inPatterns, outPatterns := readInput("input.txt")

	part1 := solvePart1(outPatterns)
	fmt.Println("Part 1:", part1)

    var sum int
    for i, input := range inPatterns {
        output := outPatterns[i]
        decoder := Decoder{input, map[int]string{}, map[string]int{}}
        decoder.determineEncoding()
        sum += decoder.decode(output)
    }

    fmt.Println("Part 2:", sum)
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
    patterns []string
	numbers  map[int]string
    encoding map[string]int
}

func (d *Decoder) findNumbers1478() []string {
	unknown := []string{}
	for _, p := range d.patterns {
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
			unknown = append(unknown, p)
		}
	}

    return unknown
}

func (d *Decoder) findNumbers369(patterns []string) []string {
	unknown := []string{}
	for _, p := range patterns {
		p_len := len(p)
		if p_len == 5 && numOverlap(d.numbers[1], p) == 2 {
			d.numbers[3] = p
		} else if p_len == 6 && numOverlap(d.numbers[7], p) == 2 {
			d.numbers[6] = p
		} else if p_len == 6 && numOverlap(d.numbers[4], p) == 4 {
			d.numbers[9] = p
		} else {
            unknown = append(unknown, p)
        }
	}

    return unknown
}

func (d *Decoder) findNumbers25(unknown2 []string) {
	for _, p := range unknown2 {
        lenP := len(p)
		if lenP == 5 {
            if numOverlap(d.numbers[6], p) == 5 {
                d.numbers[5] = p
            } else if numOverlap(d.numbers[6], p) == 4 {
                d.numbers[2] = p

            }
        } else if lenP == 6 {
			d.numbers[0] = p
		}
	}
}

func (d *Decoder) determineEncoding() {
    unknown := d.findNumbers1478()
    unknown2 := d.findNumbers369(unknown)
    d.findNumbers25(unknown2)
    d.encoding = makeEncodingMap(d.numbers)
}

func (d Decoder) decode(patterns []string) int {
    var decodedStr string
    for _, p := range patterns {
        key := sortedString(p)
        decodedStr += fmt.Sprint(d.encoding[key])
    }

    num, _ := strconv.Atoi(decodedStr)
    return num
}

func numOverlap(code string, pattern string) int {
	count := 0
	for _, char := range code {
		if strings.ContainsRune(pattern, char) {
			count++
		}
	}
	return count
}

func sortedString(str string) string {
    s := []rune(str)
    sort.Slice(s, func(i int, j int) bool { return s[i] < s[j] })

    return string(s)
}

func makeEncodingMap(m map[int]string) map[string]int {
    n := make(map[string]int, len(m))
    for k, v := range m {
        n[sortedString(v)] = k
    }
    return n
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
