use core::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone)]
enum Shape {
    Rock = 1,
    Paper = 2,
    Sciccors = 3,
}

enum GameResult {
    Victory = 6,
    Draw = 3,
    Loss = 0,
}

impl GameResult {
    fn from_character(character: &str) -> Option<GameResult> {
        match character {
            "X" => Some(GameResult::Loss),
            "Y" => Some(GameResult::Draw),
            "Z" => Some(GameResult::Victory),
            _ => None,
        }
    }
}

impl Shape {
    fn from_character(character: &str) -> Option<Shape> {
        match character {
            "A" | "X" => Some(Shape::Rock),
            "B" | "Y" => Some(Shape::Paper),
            "C" | "Z" => Some(Shape::Sciccors),
            _ => None,
        }
    }

    fn beats(&self, shape: &Shape) -> GameResult {
        match self {
            Self::Rock => match shape {
                Self::Rock => GameResult::Draw,
                Self::Paper => GameResult::Loss,
                Self::Sciccors => GameResult::Victory,
            },
            Self::Paper => match shape {
                Self::Rock => GameResult::Victory,
                Self::Paper => GameResult::Draw,
                Self::Sciccors => GameResult::Loss,
            },
            Self::Sciccors => match shape {
                Self::Rock => GameResult::Loss,
                Self::Paper => GameResult::Victory,
                Self::Sciccors => GameResult::Draw,
            },
        }
    }

    fn get_by_desired_result(&self, desired: &GameResult) -> Shape {
        match self {
            Self::Rock => match desired {
                GameResult::Victory => Self::Paper,
                GameResult::Draw => Self::Rock,
                GameResult::Loss => Self::Sciccors,
            },
            Self::Paper => match desired {
                GameResult::Victory => Self::Sciccors,
                GameResult::Draw => Self::Paper,
                GameResult::Loss => Self::Rock,
            },
            Self::Sciccors => match desired {
                GameResult::Victory => Self::Rock,
                GameResult::Draw => Self::Sciccors,
                GameResult::Loss => Self::Paper,
            },
        }
    }
}

fn main() {
    let lines = lines_from_file("input.txt");
    println!(
        "By move {}, by result {}",
        play_game_by_move(&lines),
        play_game_by_result(&lines)
    );
}

fn play_game_by_result(lines: &Vec<String>) -> i32 {
    let mut score = 0;
    let mut i = 1;
    for line in lines {
        let opponent = Shape::from_character(&line[0..1]).expect("Unable to parse move");
        let desired_result = GameResult::from_character(&line[2..3]).expect("Unable to parse move");
        let my_move = opponent.get_by_desired_result(&desired_result);
        let points = my_move.clone() as i32 + my_move.beats(&opponent) as i32;
        score += points;
        println!("{}. {:?} -> {:?}: {}", i, opponent, my_move, points);
        i += 1;
    }
    score
}

fn play_game_by_move(lines: &Vec<String>) -> i32 {
    let mut score = 0;
    let mut i = 1;
    for line in lines {
        let opponent = Shape::from_character(&line[0..1]).expect("Unable to parse move");
        let my_move = Shape::from_character(&line[2..3]).expect("Unable to parse move");
        let points = my_move.clone() as i32 + my_move.beats(&opponent) as i32;
        score += points;
        println!("{}. {:?} -> {:?}: {}", i, opponent, my_move, points);
        i += 1;
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
