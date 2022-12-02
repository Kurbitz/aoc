use std::fs::File;
use std::io::{BufRead, BufReader};

fn lines_from_file(filename: &str) -> Vec<String> {
    let file = File::open(filename).expect("No file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not read line"))
        .collect()
}

fn seperarate_elves(lines: Vec<String>) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    let mut food: Vec<i32> = Vec::new();
    for line in lines {
        if line == "" {
            result.push(food.iter().sum());
            food = Vec::new();
        } else {
            food.push(line.parse().expect("Could not parse line"));
        }
    }
    result
}

fn main() {
    let input = lines_from_file("input.txt");
    let mut elves = seperarate_elves(input);
    let largest = find_maximum_elf(&elves);
    println!(
        "The elf carrying the most calories is carrying {} calories.",
        largest
    );

    let n_max = find_maximum_n_elves(&mut elves, 3);
    let sum: i32 = n_max.iter().sum();
    println!(
        "The 3 elves carrying the most calories are carrying {} calories.",
        sum
    );
}

fn find_maximum_n_elves(elves: &Vec<i32>, n: usize) -> Vec<i32> {
    let mut sorted = elves.clone();
    let mut last_n = Vec::new();
    sorted.sort();
    for calories in sorted.iter().rev().take(n) {
        last_n.push(calories.clone());
    }
    last_n
}

fn find_maximum_elf(elves: &Vec<i32>) -> i32 {
    let mut largest = elves.first().unwrap();
    for calories in elves.iter() {
        if calories > largest {
            largest = calories;
        }
    }
    largest.clone()
}
