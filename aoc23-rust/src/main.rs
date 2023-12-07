use reqwest::blocking::Client;
use std::{fs, io};

mod day1;
mod day2;
mod day3;

use day1::Day1;
use day2::Day2;
use day3::Day3;

fn get_cookie() -> String {
    let mut cookie = String::new();
    fs::read_to_string(".env")
        .expect("Failed to read .env file")
        .lines()
        .for_each(|line| {
            if line.starts_with("COOKIE") {
                cookie = line.split("=").nth(1).unwrap().to_string();
            }
        });
    cookie.trim().to_string()
}

fn main() {
    let mut day_str = String::new();
    println!("Which day do you want to run?");
    io::stdin()
        .read_line(&mut day_str)
        .expect("Failed to read line");

    match day_str.trim().parse::<u8>() {
        Ok(day) if day > 0 && day < 26 => match day {
            1 => Day1::solve(),
            2 => Day2::solve(),
            3 => Day3::solve(),
            _ => println!("Day {} not implemented yet", day),
        },
        _ => println!("Invalid day, must be an integer between 1 and 25"),
    }
}

trait Solution {
    fn solve();

    fn get_day_input(day: u8) -> String {
        let client = Client::new();

        client
            .get(format!("https://adventofcode.com/2023/day/{}/input", day))
            .header("Cookie", format!("session={}", get_cookie()))
            .send()
            .expect("Failed to get input")
            .text()
            .expect("Failed to get input text")
    }
}
