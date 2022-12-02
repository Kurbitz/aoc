use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let lines = lines_from_file("input.txt");
    println!(
        "By move {}, by result {}",
        play_game_by_move(&lines),
        play_game_by_result(&lines)
    );
}

fn play_game_by_result(lines: &Vec<String>) -> i32 {
    let game_map = HashMap::from([
        ("A X", 0 + 3),
        ("A Y", 3 + 1),
        ("A Z", 6 + 2),
        ("B X", 0 + 1),
        ("B Y", 3 + 2),
        ("B Z", 6 + 3),
        ("C X", 0 + 2),
        ("C Y", 3 + 3),
        ("C Z", 6 + 1),
    ]);

    let mut score = 0;
    for line in lines {
        score += game_map[&line[0..]];
    }
    score
}

fn play_game_by_move(lines: &Vec<String>) -> i32 {
    let game_map = HashMap::from([
        ("A X", 3 + 1),
        ("A Y", 6 + 2),
        ("A Z", 0 + 3),
        ("B X", 0 + 1),
        ("B Y", 3 + 2),
        ("B Z", 6 + 3),
        ("C X", 6 + 1),
        ("C Y", 0 + 2),
        ("C Z", 3 + 3),
    ]);

    let mut score = 0;
    for line in lines {
        score += game_map[&line[0..]];
    }
    score
}

fn lines_from_file(filename: &str) -> Vec<String> {
    let file = File::open(filename).expect("No file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not read line"))
        .collect()
}
