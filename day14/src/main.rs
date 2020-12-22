use std::collections::HashMap;
use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt").unwrap();
    part1(&input);
}

fn part1(input: &str) {
    let mut mem: HashMap<u64, u64> = HashMap::new();
    let mut mask: Option<Mask> = None;
    for line in input.lines() {
        let op = parse_op(line);
        match op {
            Op::Mask(m) => mask = Some(m),
            Op::Mem { address, value } => {
                let mask = mask.as_ref().unwrap();
                mem.insert(address, value & mask.and | mask.or);
            }
        }
    }

    println!("Part 1: {}", mem.values().sum::<u64>())
}

fn parse_op(line: &str) -> Op {
    let (k, v) = line.split_at(line.find("=").unwrap());
    let (k, v) = (k.trim(), &v[1..].trim());
    if k == "mask" {
        let or = u64::from_str_radix(&v.replace("X", "0"), 2).unwrap();
        let and = u64::from_str_radix(&v.replace("X", "1"), 2).unwrap();
        Op::Mask(Mask { and, or })
    } else {
        Op::Mem {
            address: k[4..k.len() - 1].parse::<u64>().unwrap(),
            value: v.parse::<u64>().unwrap(),
        }
    }
}

enum Op {
    Mask(Mask),
    Mem { address: u64, value: u64 },
}

struct Mask {
    and: u64,
    or: u64,
}
