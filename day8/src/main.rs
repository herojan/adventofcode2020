use std::error::Error;
use std::fs::read_to_string;
use std::str::FromStr;

fn main() {
    let input = read_to_string("input.txt").expect("could not read input file");
    let mut ops: Vec<Op> = input.lines().map(|l| l.parse().unwrap()).collect();
    part1(&ops);
    part2(&mut ops)
}

fn part1(ops: &Vec<Op>) {
    let (acc, _) = execute(ops);
    println!("{}", acc)
}

fn part2(ops: &mut Vec<Op>) {
    for i in 0..ops.len() {
        let old = ops[i];
        match ops[i] {
            Op::JMP(n) => ops[i] = Op::NOP(n),
            Op::NOP(n) => ops[i] = Op::JMP(n),
            _ => (),
        }
        let (acc, finished) = execute(ops);
        ops[i] = old;
        if finished {
            println!("{}", acc);
            break;
        }
    }
}

fn execute(ops: &Vec<Op>) -> (i32, bool) {
    let mut seen: Vec<usize> = vec![];
    let mut i = 0;
    let mut acc = 0;
    let mut finished = true;
    while i < ops.len() {
        if seen.contains(&i) {
            finished = false;
            break;
        }
        let op = &ops[i];
        seen.push(i);
        match op {
            Op::ACC(n) => acc += n,
            Op::JMP(n) => {
                let n = n - 1;
                if n < 0 {
                    let nabs = n.abs() as usize;
                    if i >= nabs {
                        i -= nabs;
                    } else {
                        return (acc, false);
                    }
                } else {
                    i += n as usize
                }
            }
            Op::NOP(_) => (),
        }

        i += 1
    }

    (acc, finished)
}

#[derive(Debug, Copy, Clone)]
enum Op {
    ACC(i32),
    JMP(i32),
    NOP(i32),
}

impl FromStr for Op {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(' ');
        let instruction = parts.next().unwrap();
        let num: i32 = parts.next().unwrap().parse().unwrap();
        match instruction {
            "acc" => Ok(Op::ACC(num)),
            "jmp" => Ok(Op::JMP(num)),
            "nop" => Ok(Op::NOP(num)),
            _ => Err(Box::<dyn Error>::from(format!(
                "invalid instruction '{}'",
                instruction
            ))),
        }
    }
}
