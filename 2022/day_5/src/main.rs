use regex::Regex;
use std::collections::VecDeque;
use std::error::Error;
use std::vec;

#[derive(Debug, PartialEq, Clone, Copy)]
struct Move {
    amount: usize,
    from: usize,
    to: usize,
}

#[derive(Debug, PartialEq, Clone)]
struct Input {
    stacks: Vec<Vec<char>>,
    moves: VecDeque<Move>,
}

fn main() {
    let mut input = parse_input("input.txt").expect("Parsing error");
    let message = part_1(&mut input);
    println!("{}", message);
}

fn parse_input(filename: &str) -> Result<Input, Box<dyn Error>> {
    let string_file = std::fs::read_to_string(filename)?;
    let parts: Vec<&str> = string_file.split("\n\n").collect();
    assert_eq!(parts.len(), 2);
    let mut first: Vec<&str> = parts[0].split('\n').collect();
    let second: Vec<&str> = parts[1].split('\n').collect();

    let stacks = first.pop().unwrap();
    let num_of_stacks: usize = stacks[stacks.len() - 2..stacks.len() - 1].parse()?;

    let mut stacks: Vec<Vec<char>> = vec![Vec::new(); num_of_stacks];
    for line in first.iter().rev() {
        let mut j = 1;
        let chars: Vec<char> = line.chars().collect();
        for i in 0..num_of_stacks {
            let char_at_index = chars[j];
            if char_at_index.is_alphabetic() {
                stacks[i].push(line.as_bytes()[j] as char);
            }
            j += 4;
        }
    }

    let mut moves = VecDeque::from(vec![]);

    for line in second {
        let _a: Vec<usize> = line
            .chars()
            .filter(|c| c.is_digit(10))
            .map(|c| usize::from_str_radix(&c.to_string(), 10).unwrap())
            .collect();
        let re = Regex::new(r"(\d+)").unwrap();
        let a: Vec<usize> = re
            .captures_iter(line)
            .map(|cap| cap[1].parse().unwrap())
            .collect();
        if a.len() == 3 {
            moves.push_back(Move {
                amount: a[0],
                from: a[1],
                to: a[2],
            });
        } else {
            return Err("Invalid input".into());
        }
    }

    Ok(Input {
        stacks: stacks,
        moves: moves,
    })
}

fn part_1(input: &mut Input) -> String {
    for mv in input.moves.iter() {
        for _ in 0..mv.amount {
            let c = input.stacks[mv.from - 1].pop().unwrap();
            input.stacks[mv.to - 1].push(c);
        }
    }
    let mut result: Vec<char> = Vec::new();
    for c in &input.stacks {
        result.push(*c.last().unwrap());
    }

    let result: String = result.into_iter().collect();

    return result;
}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;

    use crate::{parse_input, part_1, Input, Move};

    #[test]
    fn example_parse() {
        let stacks = vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']];
        let moves = VecDeque::from(vec![
            Move {
                amount: 1,
                from: 2,
                to: 1,
            },
            Move {
                amount: 3,
                from: 1,
                to: 3,
            },
            Move {
                amount: 2,
                from: 2,
                to: 1,
            },
            Move {
                amount: 1,
                from: 1,
                to: 2,
            },
        ]);
        let input = Input {
            stacks: stacks.clone(),
            moves: moves.clone(),
        };
        let parsed = parse_input("example.txt").expect("Parsing error");
        assert_eq!(stacks, parsed.stacks);
        assert_eq!(moves, parsed.moves);
        assert_eq!(input, parsed);
    }

    #[test]
    fn example_solve_part_1() {
        let message = "CMZ";
        let mut input = parse_input("example.txt").unwrap();
        assert_eq!(message, part_1(&mut input));
    }
}
