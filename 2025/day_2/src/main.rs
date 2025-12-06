use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let (first, second) = solve(&input);
    println!("Part 1: {}", first);
    println!("Part 1: {}", second);
}

fn solve(input: &str) -> (i64, i64) {
    let mut sum_part_1 = 0;
    let mut sum_part_2 = 0;
    for ids in input.split_terminator(',') {
        let (a, b) = ids.split_once('-').unwrap();

        let start: i64 = a.parse().expect("Failed to parse start");
        let end: i64 = b.parse().expect("Failed to parse end");

        sum_part_1 += sum_of_repeats_1(start, end);
        sum_part_2 += sum_of_repeats_2(start, end);
    }
    (sum_part_1, sum_part_2)
}

fn sum_of_repeats_1(start: i64, end: i64) -> i64 {
    (start..=end).filter(|x| repeats_1(&x.to_string())).sum()
}

fn sum_of_repeats_2(start: i64, end: i64) -> i64 {
    (start..=end).filter(|x| repeats_2(&x.to_string())).sum()
}

fn repeats_1(id: &str) -> bool {
    let len = id.len();
    if len % 2 != 0 {
        false
    } else {
        let (a, b) = id.split_at(len / 2);
        a == b
    }
}

fn repeats_2(id: &str) -> bool {
    let len = id.len();
    for chunks in (2..=id.len()).filter(|x| len % x == 0) {
        let chunk_size = len / chunks;
        let first = &id[0..chunk_size];
        if (1..chunks).all(|i| &id[i * chunk_size..(i + 1) * chunk_size] == first) {
            return true;
        }
    }
    false
}

const TEST_INPUT: &str = include_str!("test.txt");

#[cfg(test)]
#[test]
fn test_part_1() {
    let (result, _) = solve(TEST_INPUT);
    assert_eq!(result, 1227775554)
}

#[cfg(test)]
#[test]
fn test_part_2() {
    let (_, result) = solve(TEST_INPUT);
    assert_eq!(result, 4174379265);
}

#[test]
fn test_repeats_1_11() {
    assert!(repeats_1("11"))
}

#[test]
fn test_repeats_1_99() {
    assert!(repeats_1("99"))
}

#[test]
fn test_repeats_1_1010() {
    assert!(repeats_1("1010"))
}

#[test]
fn test_repeats_1_1188511885() {
    assert!(repeats_1("1188511885"))
}

#[test]
fn test_repeats_1_222222() {
    assert!(repeats_1("222222"))
}

#[test]
fn test_repeats_1_1698528() {
    assert!(!repeats_1("1698528"))
}

#[test]
fn test_repeats_1_446446() {
    assert!(repeats_1("446446"))
}

#[test]
fn test_repeats_1_38593859() {
    assert!(repeats_1("38593859"))
}

#[test]
fn test_repeats_2_38593859() {
    assert!(repeats_2("38593859"))
}

#[test]
fn test_repeats_2_824824824() {
    assert!(repeats_2("824824824"))
}

#[test]
fn test_repeats_2_824824823() {
    assert!(!repeats_2("824824823"))
}
