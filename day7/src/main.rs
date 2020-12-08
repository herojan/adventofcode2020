#[macro_use]
extern crate lazy_static;
extern crate regex;

use regex::Regex;
use std::collections::HashMap;
use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt").expect("could not read input file");
    let bags: HashMap<String, Vec<(String, u32)>> = parse_input(&input);
    part1(&bags);
    part2(&bags)
}

fn part1(bags: &HashMap<String, Vec<(String, u32)>>) {
    let target = String::from("shiny gold");
    let mut count = 0;
    for k in bags.keys() {
        if find(bags, &target, k) {
            count += 1;
        }
    }

    println!("{}", count)
}

fn part2(bags: &HashMap<String, Vec<(String, u32)>>) {
    let start_point = String::from("shiny gold");
    let count = count(bags, &start_point, 2);

    println!("{}", count)
}

fn find(bags: &HashMap<String, Vec<(String, u32)>>, target: &String, start_point: &String) -> bool {
    let values = bags.get(start_point).unwrap();
    return if values.is_empty() {
        false
    } else if values.iter().any(|(v, _)| v == target) {
        true
    } else {
        values.iter().any(|(v, _)| find(bags, target, v))
    };
}

fn count(bags: &HashMap<String, Vec<(String, u32)>>, start_point: &String, total: u32) -> u32 {
    let values = bags.get(start_point).unwrap();
    return if values.is_empty() {
        0
    } else {
        values
            .iter()
            .map(|(colour, n)| n + (n * count(bags, colour, total)))
            .sum()
    };
}

fn parse_input(input: &str) -> HashMap<String, Vec<(String, u32)>> {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"(?P<num>\d+ )?(?P<colour>\w+ \w+) (?:bags|bag)").unwrap();
    }
    let mut map: HashMap<String, Vec<(String, u32)>> = HashMap::new();
    for line in input.lines() {
        let mut parts = line.split(" contain ");
        let part1 = parts.next().unwrap();
        let part2 = parts.next().unwrap();
        match RE.captures(part1) {
            Some(part1_caps) => {
                let key = part1_caps["colour"].to_string();
                let captures = RE.captures_iter(part2);
                let entry = map.entry(key.clone()).or_insert(vec![]);
                for caps in captures {
                    let num = &caps.name("num");
                    if let Some(num) = num {
                        let num: u32 = num.as_str().trim().parse().unwrap();
                        entry.push(((&caps["colour"]).to_string(), num));
                    }
                }
            }
            None => (),
        }
    }
    map
}
