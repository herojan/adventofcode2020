use std::collections::HashMap;
use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt").unwrap();
    part1(&input);
    part2(&input);
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

fn part2(input: &str) {
    let mut mem: HashMap<u64, u64> = HashMap::new();
    let mut mask: Option<Mask> = None;
    for line in input.lines() {
        let op = parse_op(line);
        match op {
            Op::Mask(m) => mask = Some(m),
            Op::Mem { address, value } => {
                let mask = mask.as_ref().unwrap();
                let num_x = mask.and.count_ones();
                let variations = 1 << num_x;
                // Use binary nums to get all possible variations
                for n in 0u64..variations {
                    let mut masked_address = address | mask.or;
                    let mut x_index = 0;
                    for n_index in 0..num_x {
                        // Find bit to flip in masked_address corresponding to an x from mask input
                        while mask.and & (1 << x_index) == 0 {
                            x_index += 1;
                        }
                        // if we see a 1 at this index of n, want to set the corresponding x to a 1 in masked_address.
                        // else set the corresponding x to a 0
                        if n & (1 << n_index) != 0 {
                            masked_address |= 1 << x_index;
                        } else {
                            masked_address &= !(1 << x_index)
                        }
                        x_index += 1;
                    }
                    mem.insert(masked_address, value);
                }
            }
        }
    }

    println!("Part 2: {}", mem.values().sum::<u64>())
}

fn parse_op(line: &str) -> Op {
    let (k, v) = line.split_at(line.find("=").unwrap());
    let (k, v) = (k.trim(), &v[1..].trim());
    if k == "mask" {
        // bit or 0, keeps that bit the same.
        // and 1, keeps that bit the same. Can use and 0 instead of 1, since or 1 = 1.
        // use and mask to keep track of which bits were an x.
        let or = u64::from_str_radix(&v.replace("X", "0"), 2).unwrap();
        let and = u64::from_str_radix(&v.replace("1", "0").replace("X", "1"), 2).unwrap();
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
