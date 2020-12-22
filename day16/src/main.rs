use std::collections::HashMap;
use std::fs::read_to_string;
use std::ops::RangeInclusive;

type Rules = HashMap<String, Vec<RangeInclusive<isize>>>;
type Ticket = Vec<isize>;

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let (mut rules, my_ticket, mut other_tickets) = parse(&input);
    let mut rate = 0;
    let mut i = 0;
    while i < other_tickets.len() {
        let ticket = &other_tickets[i];
        let mut valid = true;
        for num in ticket {
            valid = rules
                .values()
                .any(|ranges| ranges.iter().any(|range| range.contains(&num)));
            if !valid {
                rate += num;
                break;
            }
        }
        if !valid {
            &other_tickets.remove(i);
        } else {
            i += 1;
        }
    }
    println!("Part 1: {}", rate);

    let mut column_values = vec![];
    for i in 0..other_tickets[0].len() {
        let mut i_values = vec![];
        for ticket in &other_tickets {
            i_values.push(ticket[i]);
        }
        column_values.push(i_values)
    }

    let mut my_fields: HashMap<String, isize> = HashMap::new();
    let mut seen = vec![];

    loop {
        for i in 0..column_values.len() {
            if seen.contains(&i) {
                continue;
            }
            let column = &column_values[i];
            let matched_rules: Vec<&String> = rules
                .iter()
                .filter_map(|(name, ranges)| {
                    if column
                        .iter()
                        .all(|num| ranges.iter().any(|range| range.contains(&num)))
                    {
                        Some(name)
                    } else {
                        None
                    }
                })
                .collect();
            if matched_rules.len() == 1 {
                let name = matched_rules[0].to_owned();
                seen.push(i);
                rules.remove(&name);
                my_fields.insert(name, my_ticket[i]);
            }
        }
        if seen.len() == column_values.len() {
            break;
        }
    }

    let dep_product: isize = my_fields
        .iter()
        .filter_map(|(name, value)| {
            if name.starts_with("departure") {
                Some(value)
            } else {
                None
            }
        })
        .product();

    println!("Part 2: {:?}", dep_product);
}

fn parse(input: &str) -> (Rules, Ticket, Vec<Ticket>) {
    let mut lines = input.lines();
    let mut rules: Rules = HashMap::new();
    while let Some(line) = lines.next() {
        if line == "" {
            break;
        }
        let (name, rest) = line.split_at(line.find(":").unwrap());
        let rest: Vec<RangeInclusive<isize>> = rest[2..]
            .split(" or ")
            .map(|s| {
                let (lower, upper) = s.split_at(s.find("-").unwrap());
                let (lower, upper) = (lower.parse().unwrap(), upper[1..].parse().unwrap());
                lower..=upper
            })
            .collect();
        rules.insert(name.to_string(), rest);
    }
    lines.next();
    let my_ticket: Ticket = lines
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();
    lines.nth(1);
    let mut other_tickets = vec![];
    while let Some(line) = lines.next() {
        let ticket: Ticket = line.split(',').map(|s| s.parse().unwrap()).collect();
        other_tickets.push(ticket);
    }

    (rules, my_ticket, other_tickets)
}
