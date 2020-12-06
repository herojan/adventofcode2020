use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt").expect("could not read input file");
    let groups = parse_input(&input);
    part1(&groups);
    part2(&groups);
}
#[derive(Default, Debug)]
struct Group {
    answers_per_person: Vec<HashSet<char>>,
}

fn part1(groups: &Vec<Group>) {
    let sum: usize = groups
        .iter()
        .map(|g| {
            let chars: HashSet<&char> = g.answers_per_person.iter().flat_map(|a| a).collect();
            chars.len()
        })
        .sum();
    println!("{}", sum)
}

fn part2(groups: &Vec<Group>) {
    let sum: usize = groups
        .iter()
        .map(|g| {
            let person_count = g.answers_per_person.len();
            let mut combined = HashMap::new();
            for answers in &g.answers_per_person {
                for answer in answers {
                    *(combined.entry(answer).or_insert(0)) += 1;
                }
            }
            combined
                .iter()
                .filter_map(|(key, value)| {
                    if *value as usize == person_count {
                        Some(key)
                    } else {
                        None
                    }
                })
                .count()
        })
        .sum();
    println!("{}", sum)
}

fn parse_input(input: &str) -> Vec<Group> {
    let mut groups = vec![];
    groups.push(Group::default());
    for line in input.lines() {
        if line.is_empty() {
            groups.push(Group::default());
            continue;
        }
        let group = groups.last_mut().unwrap();
        let person_answers = line.chars().collect();
        group.answers_per_person.push(person_answers);
    }
    groups
}
