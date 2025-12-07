use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

const MAX_NEIGHBORS: usize = 4;

#[derive(Debug, Clone, Copy)]
enum Item {
    Empty,
    Roll,
    Accessed,
}

fn pos_from_char(c: char) -> Item {
    match c {
        '.' => Item::Empty,
        '@' => Item::Roll,
        _ => panic!("Invalid character in input"),
    }
}

fn part_1(input: &str) -> usize {
    let mut grid = parse_input(input);
    let x_len = grid[0].len() - 1;
    let y_len = grid.len() - 1;

    for i in 0..=y_len {
        for j in 0..=x_len {
            let Item::Roll = grid[i][j] else {
                continue;
            };

            if get_adjescent(&grid, i, j)
                .iter()
                .filter(|(x, y)| !matches!(grid[*x][*y], Item::Empty))
                .count()
                < MAX_NEIGHBORS
            {
                grid[i][j] = Item::Accessed;
            }
        }
    }

    let rolls = grid
        .iter()
        .flatten()
        .filter(|x| matches!(x, Item::Accessed))
        .count();
    rolls
}

fn part_2(input: &str) -> usize {
    let mut grid = parse_input(input);
    let x_len = grid[0].len() - 1;
    let y_len = grid.len() - 1;

    let mut last_rolls_sum = 0;

    loop {
        for i in 0..=y_len {
            for j in 0..=x_len {
                if !matches!(grid[i][j], Item::Roll) {
                    continue;
                }
                if get_adjescent(&grid, i, j)
                    .iter()
                    .filter(|(x, y)| matches!(grid[*x][*y], Item::Roll))
                    .count()
                    < MAX_NEIGHBORS
                {
                    grid[i][j] = Item::Accessed;
                }
            }
        }

        let rolls = grid
            .iter()
            .flatten()
            .filter(|x| matches!(x, Item::Accessed))
            .count();

        if rolls == last_rolls_sum {
            break;
        }
        last_rolls_sum = rolls;
    }
    last_rolls_sum
}

fn parse_input(input: &str) -> Vec<Vec<Item>> {
    input
        .lines()
        .map(|x| x.chars().map(pos_from_char).collect())
        .collect()
}

fn get_adjescent(grid: &Vec<Vec<Item>>, x: usize, y: usize) -> Vec<(usize, usize)> {
    let mut adjescent: Vec<(usize, usize)> = Vec::new();

    let x_max = grid.len() - 1;
    let y_max = grid[0].len() - 1;

    if x > 0 && y > 0 {
        adjescent.push((x - 1, y - 1)); //top left
    }
    if x > 0 {
        adjescent.push((x - 1, y)); //top
    }
    if x > 0 && y < y_max {
        adjescent.push((x - 1, y + 1)); //top right
    }
    if y > 0 {
        adjescent.push((x, y - 1)); //mid left
    }
    if y < y_max {
        adjescent.push((x, y + 1)); //mid right
    }
    if x < x_max && y > 0 {
        adjescent.push((x + 1, y - 1)); //bottom left
    }
    if x < x_max {
        adjescent.push((x + 1, y)); //bottom
    }
    if x < x_max && y < y_max {
        adjescent.push((x + 1, y + 1)); //bottom right
    }

    adjescent
}

#[allow(dead_code)]
const TEST_INPUT: &str = include_str!("test.txt");

#[cfg(test)]
#[test]
fn test_part_1() {
    assert_eq!(part_1(TEST_INPUT), 13)
}

#[test]
fn test_part_2() {
    assert_eq!(part_2(TEST_INPUT), 43)
}

#[test]
fn test_adjescent() {
    let grid = "..@
@@.
...";
    let grid = &parse_input(grid);
    assert_eq!(get_adjescent(grid, 1, 1).len(), 8);
    assert_eq!(get_adjescent(grid, 0, 0).len(), 3);
    assert_eq!(get_adjescent(grid, 0, 2).len(), 3);
    assert_eq!(get_adjescent(grid, 1, 2).len(), 5);
    assert_eq!(get_adjescent(grid, 2, 2).len(), 3);
}
