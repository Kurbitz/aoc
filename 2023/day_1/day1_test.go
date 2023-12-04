package main

import (
	"testing"
)

func TestSampleSolutionOne(t *testing.T) {
	lines := []string{
		"1abc2",
		"pqr3stu8vwx",
		"a1b2c3d4e5f",
		"treb7uchet",
	}
	if SolutionOne(lines) != 142 {
		t.FailNow()
	}
}

func TestSampleSolutionTwo(t *testing.T) {
	lines := []string{
		"two1nine",
		"eightwothree",
		"abcone2threexyz",
		"xtwone3four",
		"4nineeightseven2",
		"zoneight234",
		"7pqrstsixteen",
		"eightwo",
	}
	if SolutionTwo(lines) != 363 {
		t.FailNow()
	}
}
