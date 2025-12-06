use std::cmp::Ordering;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");

    println!("Part 1: {}", part_1(&input),);

    println!("Part 2: {}", part_2(&input));
}

fn part_1(input: &str) -> u64 {
    input.lines().fold(0, |acc, line| acc + max_value(line, 2))
}

fn part_2(input: &str) -> u64 {
    input.lines().fold(0, |acc, line| acc + max_value(line, 12))
}

fn max_value(line: &str, take: usize) -> u64 {
    let len = line.len();

    if take == 0 {
        return 0;
    }

    let (max_index, max) = line[0..=len - take]
        .chars()
        .map(|x| x.to_digit(10).unwrap() as u64)
        .enumerate()
        .max_by(|(_, a), (_, b)| if a == b { Ordering::Greater } else { a.cmp(b) })
        .unwrap();

    let value = max * 10_u64.pow((take - 1) as u32);

    value + max_value(&line[max_index + 1..], take - 1)
}

#[allow(dead_code)]
const TEST_INPUT: &str = include_str!("test.txt");

#[cfg(test)]
#[test]
fn test_part_1() {
    assert_eq!(part_1(TEST_INPUT), 357)
}

#[test]
fn test_part_2() {
    assert_eq!(part_2(TEST_INPUT), 3121910778619)
}
