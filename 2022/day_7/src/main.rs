use std::collections::HashMap;

fn main() {
    let input = parse_input("input.txt");
    println!("{}", part_1(input.clone()));
    println!("{}", part_2(input));
}

fn parse_input(filename: &str) -> HashMap<String, u32> {
    let mut path: Vec<String> = Vec::new();
    path.push("".to_string());
    let mut dir_sizes: HashMap<String, u32> = HashMap::new();
    let input = std::fs::read_to_string(filename).unwrap();
    let input: Vec<&str> = input.lines().collect();

    for line in input.into_iter().skip(1) {
        let parts: Vec<&str> = line.split_whitespace().collect();

        if line.starts_with("$ cd") {
            if (*parts.get(2).unwrap()) == ".." {
                path.pop();
            } else {
                path.push(parts.get(2).unwrap().to_string());
            }
        } else if line.starts_with("$ ls") || line.starts_with("dir") {
            continue;
        } else {
            let size: u32 = parts.get(0).unwrap().parse().unwrap();

            let mut trail: Vec<String> = Vec::new();
            for dir in path.iter() {
                trail.push(dir.to_string());
                let update = trail.join("/");
                dir_sizes
                    .entry(update)
                    .and_modify(|s| *s += size)
                    .or_insert(size);
            }
        }
    }
    dir_sizes
}

fn part_1(input: HashMap<String, u32>) -> u32 {
    input.values().filter(|&d| *d < 100000).sum()
}

fn part_2(input: HashMap<String, u32>) -> u32 {
    let target_size = 30000000 - (70000000 - input.get("").unwrap());
    input
        .values()
        .filter(|&d| *d >= target_size)
        .map(|f| *f)
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = parse_input("example.txt");
        assert_eq!(95437, part_1(input));
    }

    #[test]
    fn test_part_2() {
        let input = parse_input("example.txt");
        assert_eq!(24933642, part_2(input));
    }
}
