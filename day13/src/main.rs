use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt").expect("Couldn't read input file");
    let (leaving_at, bus_ids) = parse(&input);
    part1(leaving_at, &bus_ids);
    part2(&bus_ids);
}

fn part1(leaving_at: usize, bus_ids: &Vec<(usize, usize)>) {
    let (earliest_bus, wait_time) = bus_ids
        .iter()
        .map(|(_, bus_id)| (bus_id, minutes_to_wait(leaving_at, *bus_id)))
        .min_by_key(|(_, wait_time)| *wait_time)
        .unwrap();

    println!("Part 1: {}", earliest_bus * wait_time)
}

// https://en.wikipedia.org/wiki/Chinese_remainder_theorem#Search_by_sieving
// bus_ids are coprime, only test products of previous solutions.
fn part2(criteria: &Vec<(usize, usize)>) {
    let (mut t, mut step) = (1, 1);
    for (a, n) in criteria {
        loop {
            let r = (t + a).rem_euclid(*n);
            if r == 0 {
                step *= n;
                break;
            }
            t += step;
        }
    }
    println!("Part 2: {}", t)
}

fn minutes_to_wait(leaving_at: usize, bus_id: usize) -> usize {
    bus_id - (leaving_at % bus_id)
}

fn parse(input: &str) -> (usize, Vec<(usize, usize)>) {
    let mut lines = input.lines();
    let leaving_at = lines.next().unwrap().parse().unwrap();
    let bus_ids = lines
        .next()
        .unwrap()
        .split(',')
        .enumerate()
        .filter_map(|(i, part)| {
            if part != "x" {
                Some((i, part.parse().unwrap()))
            } else {
                None
            }
        })
        .collect();

    (leaving_at, bus_ids)
}
