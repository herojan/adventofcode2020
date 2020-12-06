#[macro_use]
extern crate lazy_static;
extern crate regex;

use regex::Regex;
use std::fs::File;
use std::io::Read;
use std::ops::RangeInclusive;

fn main() -> () {
    let mut input = String::new();
    File::open("input.txt")
        .expect("Could not find input file")
        .read_to_string(&mut input)
        .expect("could not read input file");
    input = Regex::new(r"\n(?P<w>\S)")
        .unwrap()
        .replace_all(&input, " $w")
        .to_string();
    input = Regex::new(r"\n\n")
        .unwrap()
        .replace_all(&input, "")
        .to_string();
    let passports = parse_passports(&input);
    part1(&passports);
    part2(&passports);
}

fn part1(passports: &Vec<Passport>) {
    let count = passports.iter().filter(|p| p.has_required_fields()).count();
    println!("{}", count);
}

fn part2(passports: &Vec<Passport>) {
    let count = passports
        .iter()
        .filter(|p| p.required_fields_valid())
        .count();
    println!("{}", count);
}

fn parse_passports(input: &str) -> Vec<Passport> {
    let mut passports = vec![];
    for line in input.lines() {
        let mut passport = Passport::default();
        for part in line.trim().split(' ') {
            let mut kv = part.trim().split(':');
            let key = kv.next().unwrap();
            let value = kv.next();
            match key {
                "byr" => passport.birth_year = value,
                "iyr" => passport.issue_year = value,
                "eyr" => passport.exp_year = value,
                "hgt" => passport.height = value,
                "hcl" => passport.hair_colour = value,
                "ecl" => passport.eye_colour = value,
                "pid" => passport.passport_id = value,
                "cid" => passport.country_id = value,
                _ => (),
            }
        }
        passports.push(passport)
    }

    passports
}

#[derive(Hash, Eq, PartialEq, Debug, Default)]
struct Passport<'a> {
    birth_year: Option<&'a str>,
    issue_year: Option<&'a str>,
    exp_year: Option<&'a str>,
    height: Option<&'a str>,
    hair_colour: Option<&'a str>,
    eye_colour: Option<&'a str>,
    passport_id: Option<&'a str>,
    country_id: Option<&'a str>,
}

impl<'a> Passport<'a> {
    fn has_required_fields(&self) -> bool {
        self.birth_year.is_some()
            && self.issue_year.is_some()
            && self.exp_year.is_some()
            && self.height.is_some()
            && self.hair_colour.is_some()
            && self.eye_colour.is_some()
            && self.passport_id.is_some()
    }
    fn required_fields_valid(&self) -> bool {
        if !self.has_required_fields() {
            return false;
        }

        let byr = valid_range(self.birth_year.unwrap(), 1920..=2002);
        let iyr = valid_range(self.issue_year.unwrap(), 2010..=2020);
        let eyr = valid_range(self.exp_year.unwrap(), 2020..=2030);
        let hgt = self.valid_height();
        let hcl = self.valid_hair_colour();
        let ecl = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
            .contains(&self.eye_colour.unwrap());
        let pid = self.valid_passport_id();

        byr && iyr && eyr && hgt && hcl && ecl && pid
    }

    fn valid_height(&self) -> bool {
        lazy_static! {
            static ref RE_HGT: Regex = Regex::new(r"^(?P<num>\d+)(?P<unit>cm|in)$").unwrap();
        }
        match RE_HGT.captures(self.height.unwrap()) {
            None => false,
            Some(captures) => {
                let num = &captures["num"];
                let unit = &captures["unit"];
                match unit {
                    "cm" => valid_range(num, 150..=193),
                    "in" => valid_range(num, 59..=76),
                    _ => panic!(format!("invalid unit {}", unit)),
                }
            }
        }
    }

    fn valid_hair_colour(&self) -> bool {
        lazy_static! {
            static ref RE_HCL: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
        }
        RE_HCL.is_match(self.hair_colour.unwrap())
    }

    fn valid_passport_id(&self) -> bool {
        lazy_static! {
            static ref RE_PID: Regex = Regex::new(r"^\d{9}$").unwrap();
        }
        RE_PID.is_match(self.passport_id.unwrap())
    }
}

fn valid_range(num: &str, range: RangeInclusive<i32>) -> bool {
    let num: Result<i32, _> = num.parse();
    num.map(|date| range.contains(&date)).unwrap_or(false)
}
