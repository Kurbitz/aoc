package main

import (
	"bufio"
	"fmt"
	"os"
	"regexp"
	"strconv"
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
}

func SolutionOne(input []string) int {
	re := regexp.MustCompile("[0-9]")
	sum := 0
	for _, line := range input {
		matching := re.FindAllString(line, -1)
		fmt.Printf("%v %v\n", matching[0], matching[len(matching)-1])
		num, err := strconv.Atoi(matching[0] + matching[len(matching)-1])
		if err != nil {
			panic(err)
		}
		sum += num
	}
	return sum
}
