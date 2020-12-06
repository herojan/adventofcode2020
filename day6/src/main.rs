use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt").expect("could not read input file");
    part1(&input);
}

fn part1(input: &str) {
    for line in input.lines() {
        println!("{}", line);
    }
}