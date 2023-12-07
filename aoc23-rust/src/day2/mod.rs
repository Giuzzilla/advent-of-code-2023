use fancy_regex::Regex;

#[derive(Debug)]
enum Extraction {
    Green(u32),
    Red(u32),
    Blue(u32),
}

fn get_extractions(line: &str) -> (u32, Vec<Extraction>) {
    let game_id = line
        .split(" ")
        .nth(1)
        .unwrap()
        .split(":")
        .nth(0)
        .unwrap()
        .parse::<u32>()
        .unwrap();
    let extractions_pat = Regex::new(r"(\d+) (red|green|blue)").unwrap();

    let extractions = extractions_pat
        .captures_iter(line)
        .map(|capture| {
            let groups = capture.unwrap();
            let number = groups.get(1).unwrap().as_str().parse::<u32>().unwrap();
            match groups.get(2).unwrap().as_str() {
                "red" => Extraction::Red(number),
                "green" => Extraction::Green(number),
                "blue" => Extraction::Blue(number),
                _ => panic!("Invalid color: {}", groups.get(2).unwrap().as_str()),
            }
        })
        .collect();

    (game_id, extractions)
}

fn first_star(input: &str) -> u32 {
    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;
    input
        .lines()
        .filter_map(|line| {
            let (game_id, extractions) = get_extractions(line);

            let valid_game = extractions
                .iter()
                .filter(|extraction| match extraction {
                    Extraction::Red(number) => number > &max_red,
                    Extraction::Green(number) => number > &max_green,
                    Extraction::Blue(number) => number > &max_blue,
                })
                .count()
                == 0;

            if valid_game {
                Some(game_id)
            } else {
                None
            }
        })
        .sum()
}

fn second_star(input: &str) -> u32 {
    let parsed = input.lines().map(get_extractions);

    parsed
        .map(|(_, extractions)| {
            let max_red = extractions
                .iter()
                .filter_map(|extraction| match extraction {
                    Extraction::Red(number) => Some(number),
                    _ => None,
                })
                .max()
                .unwrap();
            let max_green = extractions
                .iter()
                .filter_map(|extraction| match extraction {
                    Extraction::Green(number) => Some(number),
                    _ => None,
                })
                .max()
                .unwrap();

            let max_blue = extractions
                .iter()
                .filter_map(|extraction| match extraction {
                    Extraction::Blue(number) => Some(number),
                    _ => None,
                })
                .max()
                .unwrap();

            max_red * max_green * max_blue
        })
        .sum()
}

pub fn day2(input: String) {
    println!(
        "1st day - First star: {}, Second star: {}",
        first_star(&input),
        second_star(&input),
    );
}
