use std::{
    fs,
    ops::{Add, Mul},
};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    println!("Part 1 {}", part_1(&input));
    println!("Part 2 {}", part_2(&input));
}

#[derive(Clone, Copy)]
enum Operator {
    Add,
    Multiply,
}

impl Operator {
    fn from_str(s: &str) -> Option<Operator> {
        match s {
            "+" => Some(Operator::Add),
            "*" => Some(Operator::Multiply),
            _ => None,
        }
    }

    fn calculate<T: Mul<Output = T> + Add<Output = T>>(self, a: T, b: T) -> T {
        match self {
            Operator::Add => a + b,
            Operator::Multiply => a * b,
        }
    }
}

fn part_1(input: &str) -> u64 {
    let rows = input.lines().count();

    let mut numbers: Vec<Vec<_>> = input
        .lines()
        .take(rows - 1)
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse::<u64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect();

    let operators: Vec<_> = input
        .lines()
        .last()
        .unwrap()
        .split_whitespace()
        .map(|x| Operator::from_str(x).unwrap())
        .collect();

    let mut sums = numbers.pop().unwrap();

    for row in &numbers {
        for (sum, (op, &num)) in sums.iter_mut().zip(operators.iter().zip(row)) {
            *sum = op.calculate(*sum, num);
        }
    }

    sums.iter().sum()
}

fn part_2(input: &str) -> u64 {
    let mut rows: Vec<Vec<char>> = input.lines().map(|x| x.chars().collect()).collect();

    let operator_line = rows.pop().unwrap();

    let rows = rows;
    let mut groups = Vec::new();
    let mut sum = 0;

    for i in (0..operator_line.len()).rev() {
        let mut vertical_line = Vec::new();
        for row in &rows {
            let current_char = row[i];
            if current_char.is_whitespace() {
                continue;
            }

            if current_char.is_digit(10) {
                vertical_line.push(current_char);
            }
        }

        if vertical_line.len() > 0 {
            groups.push(vertical_line);
        }

        let op = operator_line[i];
        if let Some(operator) = Operator::from_str(&op.to_string()) {
            sum += groups
                .iter()
                .map(|x: &Vec<char>| x.iter().collect::<String>().parse::<u64>().unwrap())
                .reduce(|acc, e| operator.calculate(acc, e))
                .unwrap();
            groups.clear();
        }
    }

    sum
}

#[allow(dead_code)]
const TEST_INPUT: &str = include_str!("test.txt");

#[cfg(test)]
#[test]
fn test_part_1() {
    assert_eq!(part_1(TEST_INPUT), 4277556)
}

#[test]
fn test_part_2() {
    assert_eq!(part_2(TEST_INPUT), 3263827)
}
