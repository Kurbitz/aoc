use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    println!("Part 1 {}", part_1(&input));
    println!("Part 2 {}", part_2(&input));
}

fn part_1(input: &str) -> usize {
    let (ranges, ids) = parse(input);

    ids.iter()
        .filter(|id| {
            for (start, end) in &ranges {
                if **id >= *start && **id <= *end {
                    return true;
                }
            }
            false
        })
        .count()
}

fn part_2(input: &str) -> u64 {
    let overlapped_ranges = merge_ranges(input);

    overlapped_ranges
        .iter()
        .map(|(start, end)| end + 1 - start)
        .sum()
}

fn merge_ranges(input: &str) -> Vec<(u64, u64)> {
    let (mut ranges, _) = parse(input);
    ranges.sort_by(|(a, _), (b, _)| a.cmp(b));

    let mut overlapping: Vec<(u64, u64)> = Vec::new();

    overlapping.push(ranges[0]);

    for i in 1..ranges.len() {
        let (start, end) = ranges[i];
        if let Some(last) = overlapping.last_mut() {
            if start <= last.1 {
                last.1 = last.1.max(end);
            } else {
                overlapping.push((start, end));
            }
        }
    }

    overlapping
}

fn parse(input: &str) -> (Vec<(u64, u64)>, Vec<u64>) {
    let (ranges, ids) = input.split_once("\n\n").unwrap();
    let ranges: Vec<_> = ranges
        .lines()
        .map(|line| {
            let (low, high) = line.split_once("-").unwrap();
            (low.parse::<u64>().unwrap(), high.parse::<u64>().unwrap())
        })
        .collect();

    let ids: Vec<_> = ids.lines().map(|x| x.parse::<u64>().unwrap()).collect();
    (ranges, ids)
}

#[allow(dead_code)]
const TEST_INPUT: &str = include_str!("test.txt");

#[cfg(test)]
#[test]
fn test_part_1() {
    assert_eq!(part_1(TEST_INPUT), 3)
}

#[test]
fn test_part_2() {
    assert_eq!(part_2(TEST_INPUT), 14);
    let test2 = "3-5
10-14
16-20
12-18
8-23
1-2
23-25
27-30

1
5
8
11
17
32";
    assert_eq!(part_2(test2), 27);
}

#[test]
fn test_merge_ranges() {
    assert_eq!(merge_ranges(TEST_INPUT), vec![(3, 5), (10, 20)]);
    let test2 = "3-5
10-14
16-20
12-18
8-23
1-2
23-25
27-30

1
5
8
11
17
32";
    assert_eq!(merge_ranges(test2), vec![(1, 2), (3, 5), (8, 25), (27, 30)])
}
