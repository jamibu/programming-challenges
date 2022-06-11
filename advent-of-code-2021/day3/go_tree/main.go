package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

func main() {
	f, _ := os.Open("./input.txt")
	scanner := bufio.NewScanner(f)

	tree := &BinaryTree{root: &BinaryNode{}}

	// Each line contains a single binary string
	for scanner.Scan() {
		line := scanner.Text()
		tree.addString(line)
	}

	fmt.Println(tree.root)

	o2Bin := tree.findO2()
	co2Bin := tree.findCO2()

	o2, _ := strconv.ParseInt(o2Bin, 2, 64)
	co2, _ := strconv.ParseInt(co2Bin, 2, 64)

	fmt.Println(o2)
	fmt.Println(co2)
	fmt.Println(o2 * co2)
}

type BinaryTree struct {
	root *BinaryNode
}

type BinaryNode struct {
	left  *BinaryNode
	right *BinaryNode
	count int
}

func (t *BinaryTree) findO2() string {
	var str string
	node := t.root

	for {
		if node.left == nil && node.right == nil {
			break
		} else if node.left == nil {
			str += string('1')
			node = node.right
			continue
		} else if node.right == nil {
			str += string('0')
			node = node.left
			continue
		}

		if node.left.count > node.right.count {
			str += string('0')
			node = node.left
		} else {
			str += string('1')
			node = node.right
		}
	}

	return str
}

func (t *BinaryTree) findCO2() string {
	var str string
	node := t.root

	for {
		if node.left == nil && node.right == nil {
			break
		} else if node.left == nil {
			str += string('1')
			node = node.right
			continue
		} else if node.right == nil {
			str += string('0')
			node = node.left
			continue
		}

		if node.left.count <= node.right.count {
			str += string('0')
			node = node.left
		} else {
			str += string('1')
			node = node.right
		}
	}

	return str
}

func (t *BinaryTree) addString(toAdd string) {
	node := t.root
	for _, r := range toAdd {
		bit := int(r - '0')
		node = node.addBit(bit)
	}
}

func (n *BinaryNode) addBit(bit int) *BinaryNode {
	if bit == 0 {
		if n.left == nil {
			n.left = &BinaryNode{count: 0, left: nil, right: nil}
		}
		n.left.count += 1
		return n.left
	} else {
		if n.right == nil {
			n.right = &BinaryNode{count: 0, left: nil, right: nil}
		}
		n.right.count += 1
		return n.right
	}
}

// func findWidth(n *BinaryNode) int {
// 	result := 0
// 	if n == nil {
// 		return result
// 	}
// 	if n.left == nil && n.right == nil {
// 		result += 1
// 	}
// 	if n.left != nil {
// 		result += findWidth(n.left)
// 	}
// 	if n.right != nil {
// 		result += findWidth(n.right)
// 	}
// 	return result
// }
