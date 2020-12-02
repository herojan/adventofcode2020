#[macro_use]
extern crate lazy_static;
extern crate regex;

use regex::Regex;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::ops::RangeInclusive;
use std::str::FromStr;

fn main() -> () {
    let mut input = String::new();
    File::open("input.txt")
        .expect("Could not find input file")
        .read_to_string(&mut input)
        .expect("could not read input file");
    let entries: Vec<(PasswordPolicy, String)> = input
        .lines()
        .map(|line| {
            let split: Vec<&str> = line.split(":").collect();
            let password_policy: PasswordPolicy = split[0].parse().unwrap();
            let password: String = split[1].trim().to_owned();
            (password_policy, password)
        })
        .collect();
    part1(&entries);
    part2(&entries);
}

fn part1(entries: &Vec<(PasswordPolicy, String)>) {
    let mut valid_passwords: usize = 0;
    for (password_policy, password) in entries {
        let count = password
            .chars()
            .filter(|c| c == &password_policy.letter)
            .count();
        if password_policy.count_range.contains(&count) {
            valid_passwords += 1;
        }
    }

    println!("{}", valid_passwords)
}

fn part2(entries: &Vec<(PasswordPolicy, String)>) {
    let mut valid_passwords: usize = 0;
    for (password_policy, password) in entries {
        let chars: Vec<char> = password.chars().collect();
        let letter = &password_policy.letter;
        let start_range = password_policy.count_range.start() - 1;
        let end_range = password_policy.count_range.end() - 1;
        let start_match = start_range < chars.len() && chars[start_range] == *letter;
        let end_match = end_range < chars.len() && chars[end_range] == *letter;
        if (start_match && !end_match) || (!start_match && end_match) {
            valid_passwords += 1;
        }
    }

    println!("{}", valid_passwords)
}

#[derive(Hash, Eq, PartialEq, Debug)]
struct PasswordPolicy {
    count_range: RangeInclusive<usize>,
    letter: char,
}

impl FromStr for PasswordPolicy {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"^(?P<lower>\d*)-(?P<upper>\d*) (?P<letter>\w)$").unwrap();
        }
        let captures = match RE.captures(s) {
            None => {
                return Err(Box::<dyn Error>::from(format!(
                    "invalid password policy '{}'",
                    s
                )))
            }
            Some(captures) => captures,
        };

        let lower = captures["lower"].parse().unwrap();
        let upper = captures["upper"].parse().unwrap();

        return Ok(PasswordPolicy {
            count_range: lower..=upper,
            letter: captures["letter"].parse().unwrap(),
        });
    }
}
