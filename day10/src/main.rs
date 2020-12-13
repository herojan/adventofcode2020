use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt").expect("Could not read input file");
    let mut ratings: Vec<usize> = vec![0];
    let mut highest = 0;
    for line in input.lines() {
        let rating: usize = line.parse().unwrap();
        if rating > highest {
            highest = rating;
        }
        ratings.push(rating);
    }
    ratings.push(highest + 3);
    ratings.sort_unstable();
    part1(&ratings);
    part2(&ratings)
}

fn part1(ratings: &Vec<usize>) {
    let mut diffs = [0, 0, 0];
    let mut prev = ratings[0];
    for i in 1..ratings.len() {
        let rating = ratings[i];
        let diff = rating - prev;
        diffs[diff - 1] += 1;
        prev = rating;
    }
    println!("Part 1: {}", diffs[0] * diffs[2])
}

fn part2(ratings: &Vec<usize>) {
    // Since you can take any route from a previous adapter to the current one, number of routes
    // to the current adapter is the sum of the routes to the previous adapters which can reach.
    // 1 route to the first adapter.
    let mut route_counts = vec![1];
    for i in 1..ratings.len() {
        let rating = ratings[i];
        let mut reachable_count: usize = 0;
        for j in 1..=3 {
            if i >= j {
                let j_index = i - j;
                let other_rating = ratings[j_index];
                let routes_to_other = route_counts[j_index];
                if rating - other_rating <= 3 {
                    reachable_count += routes_to_other;
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        route_counts.push(reachable_count);
    }
    println!("Part 2: {}", route_counts.last().unwrap())
}
