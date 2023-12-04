package main

import (
	"bufio"
	"fmt"
	"os"
	"regexp"
	"strconv"
	"strings"
)

func main() {
	file, err := os.Open("input.txt")
	if err != nil {
		panic(err)
	}
	defer file.Close()
	scanner := bufio.NewScanner(file)
	var lines []string

	for scanner.Scan() {
		lines = append(lines, scanner.Text())
	}

	fmt.Println(SolutionOne(lines))
	fmt.Println(SolutionTwo(lines))
}

func SolutionOne(input []string) int {
	red := regexp.MustCompile("([0-9]+) red")
	green := regexp.MustCompile("([0-9]+) green")
	blue := regexp.MustCompile("([0-9]+) blue")
	sum := 0
game:
	for i, game := range input {
		prefix := fmt.Sprintf("Game %v: ", i+1)
		line := strings.TrimPrefix(game, prefix)
		subsets := strings.Split(line, ";")
		for _, subsets := range subsets {
			r := red.FindStringSubmatch(subsets)
			g := green.FindStringSubmatch(subsets)
			b := blue.FindStringSubmatch(subsets)
			reds := sums(r)
			greens := sums(g)
			blues := sums(b)

			if reds+greens+blues > 39 {
				continue game
			}

			if reds > 12 || greens > 13 || blues > 14 {
				continue game
			}
		}

		sum += i + 1
	}
	return sum
}

func SolutionTwo(input []string) int {
	red := regexp.MustCompile("([0-9]+) red")
	green := regexp.MustCompile("([0-9]+) green")
	blue := regexp.MustCompile("([0-9]+) blue")
	sum := 0

	for i, game := range input {
		prefix := fmt.Sprintf("Game %v: ", i+1)
		line := strings.TrimPrefix(game, prefix)
		subsets := strings.Split(line, ";")
		maxReds := 0
		maxGreens := 0
		maxBlues := 0

		for _, subsets := range subsets {
			r := red.FindStringSubmatch(subsets)
			g := green.FindStringSubmatch(subsets)
			b := blue.FindStringSubmatch(subsets)
			reds := sums(r)
			greens := sums(g)
			blues := sums(b)

			maxReds = max(reds, maxReds)
			maxGreens = max(greens, maxGreens)
			maxBlues = max(blues, maxBlues)

		}
		power := maxReds * maxGreens * maxBlues
		sum += power
	}
	return sum
}

func sums(sa []string) int {
	if len(sa) != 0 {
		res, _ := strconv.Atoi(sa[1])
		return res

	}
	return 0
}

func max(a, b int) int {
	if a > b {
		return a
	}
	return b
}
