use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("");
    let (first, second) = solve(input);
    println!("Part 1: {}", first);
    println!("Part 1: {}", second);
}

fn solve(input: String) -> (i64, i64) {
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
    (start..=end).filter(|x| repeats_1(x.to_string())).sum()
}

fn sum_of_repeats_2(start: i64, end: i64) -> i64 {
    (start..=end).filter(|x| repeats_2(x.to_string())).sum()
}

fn repeats_1(id: String) -> bool {
    let len = id.len();
    if len % 2 != 0 {
        false
    } else {
        let (a, b) = id.split_at(len / 2);
        *a == *b
    }
}

fn repeats_2(id: String) -> bool {
    let len = id.len();
    let chunk_sizes = (2..=id.len()).filter(|x| len % x == 0).map(|x| len / x);

    for chunk_size in chunk_sizes {
        let chars: Vec<char> = id.chars().collect();
        let chunks: Vec<&[char]> = chars.chunks(chunk_size).collect();
        let strings: Vec<String> = chunks.iter().map(|x| x.iter().collect()).collect();

        let all_same = strings.windows(2).map(|x| &x[0] == &x[1]).all(|x| x);
        if all_same {
            return true;
        }
    }
    false
}

#[cfg(test)]
#[test]
fn test_part_1() -> Result<(), i64> {
    let input = fs::read_to_string("test.txt").expect("");

    let (result, _) = solve(input);
    match result == 1227775554 {
        true => Ok(()),
        false => Err(result),
    }
}

#[cfg(test)]
#[test]
fn test_part_2() -> Result<(), i64> {
    let input = fs::read_to_string("test.txt").expect("");

    let (_, result) = solve(input);
    match result == 4174379265 {
        true => Ok(()),
        false => Err(result),
    }
}

#[test]
fn test_repeats_1_11() -> Result<(), bool> {
    match repeats_1(String::from("11")) {
        true => Ok(()),
        false => Err(false),
    }
}

#[test]
fn test_repeats_1_99() -> Result<(), bool> {
    match repeats_1(String::from("99")) {
        true => Ok(()),
        false => Err(false),
    }
}

#[test]
fn test_repeats_1_1010() -> Result<(), bool> {
    match repeats_1(String::from("1010")) {
        true => Ok(()),
        false => Err(false),
    }
}

#[test]
fn test_repeats_1_1188511885() -> Result<(), bool> {
    match repeats_1(String::from("1188511885")) {
        true => Ok(()),
        false => Err(false),
    }
}

#[test]
fn test_repeats_1_222222() -> Result<(), bool> {
    match repeats_1(String::from("222222")) {
        true => Ok(()),
        false => Err(false),
    }
}

#[test]
fn test_repeats_1_1698528() -> Result<(), bool> {
    match repeats_1(String::from("1698528")) {
        false => Ok(()),
        true => Err(false),
    }
}

#[test]
fn test_repeats_1_446446() -> Result<(), bool> {
    match repeats_1(String::from("446446")) {
        true => Ok(()),
        false => Err(false),
    }
}

#[test]
fn test_repeats_1_38593859() -> Result<(), bool> {
    match repeats_1(String::from("38593859")) {
        true => Ok(()),
        false => Err(false),
    }
}

#[test]
fn test_repeats_2_38593859() -> Result<(), bool> {
    match repeats_2(String::from("38593859")) {
        true => Ok(()),
        false => Err(false),
    }
}

#[test]
fn test_repeats_2_824824824() -> Result<(), bool> {
    match repeats_2(String::from("824824824")) {
        true => Ok(()),
        false => Err(false),
    }
}

#[test]
fn test_repeats_2_824824823() -> Result<(), bool> {
    match repeats_2(String::from("824824823")) {
        false => Ok(()),
        true => Err(false),
    }
}
