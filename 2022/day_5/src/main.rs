use std::fs::File;
use std::io::{BufRead, BufReader};
use std::vec;


fn main() {}

fn parse_input(filename: &str) -> Vec<Vec<char>> {

    vec![vec!['c']]
}

fn part_1(stacks: Vec<Vec<char>>) -> String {
    String::from("")
}

#[cfg(test)]
mod tests {
    use crate::{parse_input, part_1};

    #[test]
    fn example_parse() {
        let stacks = vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']];
        assert_eq!(stacks, parse_input("example.txt"));
    }

    #[test]
    fn example_solve() {
        let stacks = vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']];
        let message = "CMZ";
        assert_eq!(message, part_1(stacks));
    }
}
