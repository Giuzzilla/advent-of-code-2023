use std::cmp::min;
use std::collections::HashSet;

use super::Solution;

#[derive(Debug)]
struct Extraction {
    winning_cards: HashSet<u32>,
    my_cards: HashSet<u32>,
}

fn get_extractions(input: &str) -> Vec<Extraction> {
    input
        .lines()
        .map(|line| {
            let sets_of_cards = line
                .split(": ")
                .nth(1)
                .unwrap()
                .split(" | ")
                .map(|cards_str| {
                    cards_str
                        .split(" ")
                        .filter_map(|card_str| {
                            let stripped_str = card_str.replace(" ", "");
                            if stripped_str != "" {
                                Some(stripped_str.parse::<u32>().unwrap())
                            } else {
                                None
                            }
                        })
                        .collect()
                })
                .collect::<Vec<HashSet<u32>>>();

            let [winning_cards, my_cards] = TryInto::<[_; 2]>::try_into(sets_of_cards)
                .expect(format!("More than two blocks of cards found in line {}", line).as_str());
            Extraction {
                winning_cards,
                my_cards,
            }
        })
        .collect()
}

fn first_star(input: &str) -> u32 {
    let extractions = get_extractions(input);

    extractions
        .iter()
        .map(|extraction| {
            let common = extraction.my_cards.intersection(&extraction.winning_cards);

            let common_count = common.count();

            if common_count == 0 {
                0
            } else {
                2_u32.pow((common_count - 1).try_into().unwrap())
            }
        })
        .sum()
}

fn second_star(input: &str) -> u32 {
    let extractions = get_extractions(input);

    let mut counters = vec![1; extractions.len()];

    for (i, extraction) in extractions.iter().enumerate() {
        let common: HashSet<_> = extraction
            .my_cards
            .intersection(&extraction.winning_cards)
            .collect();

        if i + 1 < extractions.len() {
            for k in (i + 1)..min(i + 1 + common.len(), extractions.len()) {
                counters[k] += counters[i];
            }
        }
    }

    counters.iter().sum()
}

pub struct Day4 {}

impl Solution for Day4 {
    fn solve() {
        let input = Self::get_day_input(4);

        println!(
            "4th day - First star: {}, Second star: {}",
            first_star(&input),
            second_star(&input),
        );
    }
}
