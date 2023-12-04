package main

import "testing"

func TestSolutionOne(t *testing.T) {
	lines := []string{
		"467..114..",
		"...*......",
		"..35..633.",
		"......#...",
		"617*......",
		".....+.58.",
		"..592.....",
		"......755.",
		"...$.*....",
		".664.598..",
	}
	if SolutionOne(lines, 10) != 4361 {
		t.FailNow()
	}
}
