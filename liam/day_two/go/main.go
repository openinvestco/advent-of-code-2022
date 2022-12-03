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
	dat, err := os.ReadFile("../input.txt")
	check(err)
	games := strings.Split(string(dat), "\n")
	score := 0
	for _, game := range games {
		var move int
		switch game[2] {
		case 'Y':
			tie_points := []int{1, 2, 3}
			move = tie_points[int(game[0])-65]
			score += move + 3 // Tie
		case 'Z':
			win_points := []int{2, 3, 1}
			move = win_points[int(game[0])-65]
			score += move + 6 // Win
		default:
			lose_points := []int{3, 1, 2}
			move = lose_points[int(game[0])-65]
			score += move // Lose
		}

	}
	fmt.Printf("Total Score: %d\n", score)

}
