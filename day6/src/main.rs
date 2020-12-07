use std::collections::HashSet;
use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt").expect("could not read input file");
    let groups = parse_input(&input);
    part1(&groups);
    part2(&groups);
}

#[derive(Default, Debug)]
struct Group {
    answers_per_person: Vec<Person>,
}
#[derive(Default, Debug)]
struct Person {
    answers: HashSet<char>,
}

fn part1(groups: &Vec<Group>) {
    let sum: usize = groups
        .iter()
        .map(|g| {
            let chars: HashSet<&char> = g
                .answers_per_person
                .iter()
                .flat_map(|a| &a.answers)
                .collect();
            chars.len()
        })
        .sum();
    println!("{}", sum)
}

fn part2(groups: &Vec<Group>) {
    let sum: usize = groups
        .iter()
        .map(|g| {
            let acc = g.answers_per_person.first().unwrap().answers.clone();
            let chars = g
                .answers_per_person
                .iter()
                .fold(acc, |b, a| b.intersection(&a.answers).copied().collect());
            chars.len()
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
        let person = Person {
            answers: line.chars().collect(),
        };
        group.answers_per_person.push(person);
    }
    groups
}
