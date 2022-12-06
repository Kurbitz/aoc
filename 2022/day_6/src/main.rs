use std::collections::HashSet;

fn main() {
    let code = parse_input("input.txt");
    println!("Part 1: {}", find_marker(&code, 4).unwrap());
    println!("Part 2: {}", find_marker(&code, 14).unwrap());
}

fn parse_input(filename: &str) -> String {
    std::fs::read_to_string(filename).unwrap()
}

fn find_marker(input: &str, marker_length: usize) -> Option<usize> {
    let input: Vec<char> = input.chars().collect();
    let windows = input.windows(marker_length);

    for (i, window) in windows.enumerate() {
        let set: HashSet<&char> = HashSet::from_iter(window.iter());
        if set.len() == marker_length {
            return Some(i + marker_length);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_part_1_1() {
        assert_eq!(5, find_marker("bvwbjplbgvbhsrlpgdmjqwftvncz", 4).unwrap());
    }

    #[test]
    fn solve_part_1_2() {
        assert_eq!(6, find_marker("nppdvjthqldpwncqszvftbrmjlhg", 4).unwrap());
    }

    #[test]
    fn solve_part_1_3() {
        assert_eq!(
            10,
            find_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4).unwrap()
        );
    }

    #[test]
    fn solve_part_1_4() {
        assert_eq!(
            11,
            find_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4).unwrap()
        );
    }

    #[test]
    fn solve_part_2_1() {
        assert_eq!(
            19,
            find_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14).unwrap()
        );
    }

    #[test]
    fn solve_part_2_2() {
        assert_eq!(23, find_marker("bvwbjplbgvbhsrlpgdmjqwftvncz", 14).unwrap());
    }

    #[test]
    fn solve_part_2_3() {
        assert_eq!(23, find_marker("nppdvjthqldpwncqszvftbrmjlhg", 14).unwrap());
    }

    #[test]
    fn solve_part_2_4() {
        assert_eq!(
            29,
            find_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14).unwrap()
        );
    }

    #[test]
    fn solve_part_2_5() {
        assert_eq!(
            26,
            find_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14).unwrap()
        );
    }
}
