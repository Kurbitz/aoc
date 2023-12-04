package main

import (
	"bufio"
	"fmt"
	"os"
	"regexp"
	"strconv"
	"unicode"
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
	fmt.Println(SolutionOne(lines, 140))
}

func SolutionOne(input []string, n int) int {
	symbolMap := make([][]int, n+2)
	for i := range symbolMap {
		symbolMap[i] = make([]int, n+2)
	}

	for ii, line := range input {
		for jj, c := range line {
			i := ii + 1
			j := jj + 1
			if unicode.IsDigit(c) || c == '.' {
				if symbolMap[i][j] != 1 {
					symbolMap[i][j] = 0
				}
			} else {
				symbolMap[i][j] = 2
				symbolMap[i][j+1] = 1
				symbolMap[i+1][j+1] = 1
				symbolMap[i+1][j] = 1
				symbolMap[i+1][j-1] = 1
				symbolMap[i][j-1] = 1
				symbolMap[i-1][j-1] = 1
				symbolMap[i-1][j] = 1
				symbolMap[i-1][j+1] = 1

			}
		}
	}
	sum := 0
	for i, line := range input {
		r := regexp.MustCompile("[0-9]+")
		indexes := r.FindAllStringIndex(line, -1)
		numbers := r.FindAllString(line, -1)
	matches:
		for matchIndex, num := range indexes {
			for j := num[0]; j < num[1]; j++ {
				if symbolMap[i+1][j+1] == 1 {
					num, _ := strconv.Atoi(numbers[matchIndex])
					sum += num
					continue matches
				}
			}
		}
	}

	return sum
}
