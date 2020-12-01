use std::fs::File;
use std::io::Read;
use std::ops::Index;

fn main() -> () {
    let mut input = String::new();
    File::open("input.txt")
        .expect("Could not find input file")
        .read_to_string(&mut input)
        .expect("could not read input file");
    let expenses: Vec<u32> = input
        .lines()
        .map(|line| {
            line.parse()
                .expect(format!("input line {} is not a u32", line).as_str())
        })
        .collect();
    part1(&expenses);
    part2(&expenses)
}

fn part1(expenses: &Vec<u32>) {
    let mut product: u32 = 0;
    for i in 0..expenses.len() {
        for j in 0..expenses.len() {
            if i == j {
                continue;
            } else {
                let i_expense = expenses.index(i);
                let j_expense = expenses.index(j);
                if i_expense + j_expense == 2020 {
                    product = i_expense * j_expense;
                    break;
                }
            }
        }
    }
    println!("{}", product);
}

fn part2(expenses: &Vec<u32>) {
    let mut product: u32 = 0;
    for i in 0..expenses.len() {
        for j in 0..expenses.len() {
            for k in 0..expenses.len() {
                if i == j || j == k || i == k {
                    continue;
                } else {
                    let i_expense = expenses.index(i);
                    let j_expense = expenses.index(j);
                    let k_expense = expenses.index(k);
                    if i_expense + j_expense + k_expense == 2020 {
                        product = i_expense * j_expense * k_expense;
                        break;
                    }
                }
            }
        }
    }
    println!("{}", product);
}
