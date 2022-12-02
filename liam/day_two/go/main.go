package main

import (
	"fmt"
	"os"
	"strings"
)

func main() {
	partOne()
	partTwo()
}

func check(e error) {
	if e != nil {
		panic(e)
	}
}

func partOne() {
	dat, err := os.ReadFile("../input.txt")
	check(err)
	games := strings.Split(string(dat), "\n")
	score := 0
	for _, game := range games {
		switch int(game[2]) - int(game[0]) {
		case 23:
			score += int(game[2]) - 84 // Tie
		case 24, 21:
			score += int(game[2]) - 81 // Win
		default:
			score += int(game[2]) - 87 // Lose
		}
	}
	fmt.Printf("Total Score: %d\n", score)
}

func partTwo() {

}
