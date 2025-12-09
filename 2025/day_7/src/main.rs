use std::{collections::HashSet, fs};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("Part 1: {}", part_1(&input));
}

fn part_1(input: &str) -> u32 {
    let mut beam_ind: HashSet<usize> = HashSet::new();

    let mut sum = 0;
    for (_, line) in input
        .lines()
        .enumerate()
        .skip(1)
        .filter(|(i, _)| i % 2 == 0)
    {
        for (j, c) in line.chars().enumerate() {
            if beam_ind.is_empty() && c == '^' {
                beam_ind.insert(j + 1);
                beam_ind.insert(j - 1);
                sum += 1;
            } else if c == '^' && beam_ind.contains(&j) {
                // split
                beam_ind.insert(j + 1);
                beam_ind.insert(j - 1);
                beam_ind.remove(&j);
                sum += 1;
            }
        }
    }
    sum
}

#[allow(dead_code)]
const TEST_INPUT: &str = include_str!("test.txt");

#[cfg(test)]
#[test]
fn test_part_1() {
    assert_eq!(part_1(TEST_INPUT), 21)
}
