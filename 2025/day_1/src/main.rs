use std::fs;

fn main() {
    let content = fs::read_to_string("input.txt").expect("Could not read file");

    println!("Password is {}", part_1(content.clone()));
    println!("0x434C49434B password is {}", part_2(content))
}

fn part_1(input: String) -> i32 {
    let mut sum = 50;
    let mut zeros = 0;

    for line in input.lines() {
        let delta = parse_line(line);

        sum = (sum + delta).rem_euclid(100);

        if sum == 0 {
            zeros += 1;
        }
    }
    zeros
}

fn part_2(input: String) -> i32 {
    let mut sum = 50;
    let mut zeros = 0;

    for line in input.lines() {
        let delta = parse_line(line);

        let (a, b) = part_2_internal(sum, delta);

        sum = a;
        zeros += b;
    }
    zeros
}

fn parse_line(line: &str) -> i32 {
    let delta: i32 = line[1..].parse().expect("");
    match &line[0..1] {
        "R" => delta,
        "L" => delta * -1,
        _ => 0,
    }
}

fn part_2_internal(sum: i32, delta: i32) -> (i32, i32) {
    let part_sum = sum + (delta % 100);
    let last_sum = sum;
    let mut passes = 0;

    passes += (delta / 100).abs();

    let new_sum = part_sum.rem_euclid(100);

    if new_sum == 0 && last_sum != 0 {
        passes += 1;
    }

    if part_sum > 100 || part_sum < 0 && sum != 0 {
        passes += 1;
    }
    println!("{sum}\t{delta}\t{new_sum}\t{passes}");
    (new_sum, passes)
}

#[cfg(test)]
#[test]
fn test_part_1() -> Result<(), i32> {
    let content = fs::read_to_string("test.txt").expect("Could not read file");

    let result = part_1(content);
    if result == 3 {
        return Ok(());
    }

    Err(result)
}

#[test]
fn test_part_2() -> Result<(), i32> {
    let content = fs::read_to_string("test.txt").expect("Could not read file");

    let result = part_2(content);
    if result == 6 {
        return Ok(());
    }

    Err(result)
}

#[test]
fn test_add_pass_once_left() -> Result<(), i32> {
    let (_, passes) = part_2_internal(50, -68);

    if passes == 1 {
        return Ok(());
    }
    Err(passes)
}

#[test]
fn test_add_pass_once_right() -> Result<(), i32> {
    let (_, passes) = part_2_internal(52, 48);

    if passes == 1 {
        return Ok(());
    }
    Err(passes)
}

#[test]
fn test_add_one_rotation_from_zero() -> Result<(), i32> {
    let (_, passes) = part_2_internal(0, 100);

    if passes == 1 {
        return Ok(());
    }
    Err(passes)
}

#[test]
fn test_add_one_negative_rotation_from_zero() -> Result<(), i32> {
    let (_, passes) = part_2_internal(0, -100);

    if passes == 1 {
        return Ok(());
    }
    Err(passes)
}

#[test]
fn test_add_two_rotations() -> Result<(), i32> {
    let (_, passes) = part_2_internal(50, 200);

    if passes == 2 {
        return Ok(());
    }
    Err(passes)
}

#[test]
fn test_left_from_zero() -> Result<(), i32> {
    let (_, passes) = part_2_internal(0, -5);

    if passes == 0 {
        return Ok(());
    }
    Err(passes)
}

#[test]
fn test_right_from_zero() -> Result<(), i32> {
    let (_, passes) = part_2_internal(0, 1);

    if passes == 0 {
        return Ok(());
    }
    Err(passes)
}
