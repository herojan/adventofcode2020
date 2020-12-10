use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt").expect("Could not read input file");

    part2(&input, part1(&input).unwrap());
}

fn part1(input: &str) -> Option<i64> {
    let (mut preamble, nums) = parse1(input, 25);
    for num in nums {
        if !check(&preamble, num) {
            println!("Found {}", num);
            return Some(num);
        }
        preamble.push(num);
        preamble.remove(0);
    }
    return None;
}

fn part2(input: &str, target: i64) {
    let nums = parse2(input, 25);
    let mut contiguous_nums = vec![nums[0], nums[1]];
    let mut total: i64 = contiguous_nums.iter().sum();
    for i in 2..nums.len() {
        let num = nums[i];
        let mut new_total = total + num;
        if new_total == target {
            contiguous_nums.push(num);
            println!(
                "{}",
                contiguous_nums.iter().min().unwrap() + contiguous_nums.iter().max().unwrap()
            );
            break;
        } else if new_total < target {
            contiguous_nums.push(num);
            total = new_total;
        } else {
            while new_total > target {
                let rem = contiguous_nums.remove(0);
                total -= rem;
                new_total -= rem;
            }

            if new_total == target {
                contiguous_nums.push(num);
                println!(
                    "{}",
                    contiguous_nums.iter().min().unwrap() + contiguous_nums.iter().max().unwrap()
                );
                break;
            }

            total = new_total;
            contiguous_nums.push(num);
        }
    }
}

fn check(preamble: &Vec<i64>, num: i64) -> bool {
    let mut has_match = false;
    for i in 0..preamble.len() {
        for j in 1..preamble.len() {
            if i != j {
                if (preamble[i] + preamble[j]) == num {
                    has_match = true;
                    break;
                }
            }
        }
    }

    has_match
}

fn parse1(input: &str, preamble_length: usize) -> (Vec<i64>, Vec<i64>) {
    let mut preamble = vec![];
    let mut nums = vec![];
    let mut lines = input.lines();
    for _ in 0..preamble_length {
        let num: i64 = lines.next().unwrap().parse().unwrap();
        preamble.push(num)
    }
    while let Some(line) = lines.next() {
        let num: i64 = line.parse().unwrap();
        nums.push(num)
    }

    (preamble, nums)
}

fn parse2(input: &str, preamble_length: usize) -> Vec<i64> {
    let (mut preamble, mut nums) = parse1(input, preamble_length);

    preamble.append(&mut nums);
    preamble
}
