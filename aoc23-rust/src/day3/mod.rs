use std::collections::HashSet;

use super::Solution;
use fancy_regex::Regex;

#[derive(PartialEq, Hash, Eq, Copy, Clone)]
struct Number {
    value: u32,
    start: usize,
    end: usize,
    line_idx: usize,
}

#[derive(PartialEq, Hash, Eq, Copy, Clone)]
enum Element {
    Empty,
    Number(Number),
    Symbol(char),
}

type Grid = Vec<Vec<Element>>;

fn get_grid(input: &str) -> Grid {
    let pattern = Regex::new(r"(\d+)|(\.)|(.)").unwrap();
    input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            pattern
                .captures_iter(line)
                .flat_map(|capture| {
                    let groups = capture.unwrap();
                    if let Some(number) = groups.get(1) {
                        let number_element = Element::Number(Number {
                            value: number.as_str().parse::<u32>().unwrap(),
                            start: number.start(),
                            end: number.end(),
                            line_idx: i,
                        });

                        vec![number_element; number.end() - number.start()]
                    } else if groups.get(2).is_some() {
                        vec![Element::Empty]
                    } else {
                        vec![Element::Symbol(
                            groups.get(3).unwrap().as_str().chars().next().unwrap(),
                        )]
                    }
                })
                .collect()
        })
        .collect()
}

fn get_neighbours(grid: &Grid, i: usize, j: usize) -> HashSet<Element> {
    let mut neighbours = HashSet::new();

    let first_row = i == 0;
    let last_row = i == grid.len() - 1;
    let first_col = j == 0;
    let last_col = j == grid[i].len() - 1;

    if !first_row {
        neighbours.insert(grid[i - 1][j]);
    }
    if !last_row {
        neighbours.insert(grid[i + 1][j]);
    }
    if !first_col {
        neighbours.insert(grid[i][j - 1]);
    }
    if !last_col {
        neighbours.insert(grid[i][j + 1]);
    }

    if !first_row && !first_col {
        neighbours.insert(grid[i - 1][j - 1]);
    }
    if !first_row && !last_col {
        neighbours.insert(grid[i - 1][j + 1]);
    }
    if !last_row && !first_col {
        neighbours.insert(grid[i + 1][j - 1]);
    }
    if !last_row && !last_col {
        neighbours.insert(grid[i + 1][j + 1]);
    }

    neighbours
}

fn first_star(input: &str) -> u32 {
    let grid = get_grid(input);

    let mut numbers_to_sum: HashSet<Number> = HashSet::new();

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if let Element::Number(num) = grid[i][j] {
                let has_symbols_nearby = get_neighbours(&grid, i, j)
                    .iter()
                    .any(|neighbour| matches!(neighbour, Element::Symbol(_)));
                if has_symbols_nearby {
                    numbers_to_sum.insert(num);
                }
            }
        }
    }

    numbers_to_sum.iter().map(|n| n.value).sum()
}

fn second_star(input: &str) -> u32 {
    let grid = get_grid(input);

    let mut sum = 0;

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if let Element::Symbol('*') = grid[i][j] {
                let neighbours = get_neighbours(&grid, i, j);
                let numbers = neighbours
                    .iter()
                    .filter_map(|neighbour| match neighbour {
                        Element::Number(number) => Some(number.value),
                        _ => None,
                    })
                    .collect::<Vec<u32>>();

                if let [number1, number2] = numbers.as_slice() {
                    sum += number1 * number2;
                }
            }
        }
    }

    sum
}

pub struct Day3 {}

impl Solution for Day3 {
    fn solve() {
        let input = Self::get_day_input(3);

        println!(
            "3rd day - First star: {}, Second star: {}",
            first_star(&input),
            second_star(&input)
        );
    }
}
