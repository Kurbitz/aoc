use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

static PRIORITY: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn main() {
    let lines = lines_from_file("input.txt");
    part_1(lines.clone());
    part_2(lines);
}

fn part_1(lines: Vec<String>) -> usize {
    let mut sum = 0;
    let a = lines
        .iter()
        .map(|l| l.split_at(l.len() / 2))
        .map(|l| l.1.chars().filter(|c| l.0.contains(*c)));
    for (first, second) in lines.iter().map(|l| l.split_at(l.len() / 2)) {
        for item in second.chars() {
            if first.contains(item) {
                sum += PRIORITY.find(item).expect("Unexpected char") + 1;
                break;
            }
        }
    }
    println!("{}", sum);
    sum
}

fn part_2(lines: Vec<String>) -> usize {
    let mut sum = 0;
    for line in lines.chunks(3) {
        let first: HashSet<char> = line[0].chars().collect();
        let second: HashSet<char> = line[1].chars().collect();
        let intersection: HashSet<&char> = first.intersection(&second).collect();
        for item in line[2].chars() {
            if intersection.contains(&item) {
                sum += PRIORITY.find(item).expect("Unexpected char") + 1;
                break;
            }
        }
    }
    println!("{}", sum);
    sum
}

fn lines_from_file(filename: &str) -> Vec<String> {
    let file = File::open(filename).expect("No file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not read line"))
        .collect()
}
