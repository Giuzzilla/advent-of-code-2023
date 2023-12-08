use super::Solution;
use fancy_regex::Regex;

fn sum_first_and_last(iter: Vec<[u32; 2]>) -> u32 {
    iter.iter()
        .map(|[digit1, digit2]| format!("{}{}", digit1, digit2).parse::<u32>().unwrap())
        .sum()
}

fn first_star(input: &str) -> u32 {
    sum_first_and_last(
        input
            .lines()
            .map(|line: &str| {
                match line
                    .chars()
                    .filter_map(|c| c.to_digit(10))
                    .collect::<Vec<u32>>()
                    .as_slice()
                {
                    [digit1, .., digit2] => [*digit1, *digit2],
                    [digit1] => [*digit1, *digit1],
                    _ => panic!("Invalid line: {}", line),
                }
            })
            .collect(),
    )
}

fn second_star(input: &str) -> u32 {
    let first_pattern = Regex::new(r"(\d|one|two|three|four|five|six|seven|eight|nine)").unwrap();
    let second_pattern =
        Regex::new(r".*(\d|one|two|three|four|five|six|seven|eight|nine)").unwrap();

    let lines_of_digits = input
        .lines()
        .map(|line: &str| {
            let first_digit = first_pattern
                .captures(line)
                .unwrap()
                .unwrap()
                .get(1)
                .unwrap()
                .as_str();
            let second_digit = second_pattern
                .captures(line)
                .unwrap()
                .unwrap()
                .get(1)
                .unwrap()
                .as_str();

            [first_digit, second_digit].map(|digit| match digit {
                "one" => 1,
                "two" => 2,
                "three" => 3,
                "four" => 4,
                "five" => 5,
                "six" => 6,
                "seven" => 7,
                "eight" => 8,
                "nine" => 9,
                _ => digit.parse::<u32>().unwrap(),
            })
        })
        .collect();

    sum_first_and_last(lines_of_digits)
}

pub struct Day1 {}

impl Solution for Day1 {
    fn solve() {
        let input = Self::get_day_input(1);

        println!(
            "1st day - First star: {}, Second star: {}",
            first_star(&input),
            second_star(&input)
        );
    }
}
