package main

import (
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
	strings.Split(string(dat), "\n")
}

func partTwo() {
	dat, err := os.ReadFile("../input.txt")
	check(err)
	strings.Split(string(dat), "\n")
}
