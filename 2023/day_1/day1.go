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
	re := regexp.MustCompile("[0-9]")
	sum := 0
	for _, line := range input {
		matching := re.FindAllString(line, -1)
		num, err := strconv.Atoi(matching[0] + matching[len(matching)-1])
		if err != nil {
			panic(err)
		}
		sum += num
	}
	return sum
}

func SolutionTwo(input []string) int {
	re := regexp.MustCompile("(one|two|three|four|five|six|seven|eight|nine|[0-9])")

	sum := 0
	for _, line := range input {
		MakeNumbersSafe(&line)
		matching := re.FindAllString(line, -1)
		num, _ := strconv.Atoi(matching[0] + matching[len(matching)-1])
		sum += num
	}
	return sum
}

var safeNumbers = map[string]string{
	"one":   "o1e",
	"two":   "t2o",
	"three": "t3e",
	"four":  "f4r",
	"five":  "f5e",
	"six":   "s6x",
	"seven": "s7n",
	"eight": "e8t",
	"nine":  "n9e",
}

func MakeNumbersSafe(s *string) {
	for old, new := range safeNumbers {
		*s = strings.ReplaceAll(*s, old, new)
	}
}
