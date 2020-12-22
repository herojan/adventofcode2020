use std::collections::HashMap;
use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt").unwrap();
    println!("Part 1: {:?}", find_last_num(&input, 2020));
    println!("Part 2: {:?}", find_last_num(&input, 30000000));
}

fn find_last_num(input: &str, target_turn_num: u32) -> u32 {
    let start_numbers: Vec<u32> = input.split(',').map(|s| s.parse().unwrap()).collect();
    let mut spoken_nums: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut last_num = 0;
    let mut turn_num = 1;
    for &num in &start_numbers {
        spoken_nums.insert(num, vec![turn_num]);
        last_num = num;
        turn_num += 1;
    }
    for _ in turn_num..=target_turn_num {
        let turn_nums = spoken_nums.get_mut(&last_num).unwrap();
        let response = if turn_nums.len() > 1 {
            turn_nums[turn_nums.len() - 1] - turn_nums[turn_nums.len() - 2]
        } else {
            0
        };
        if turn_nums.len() > 2 {
            turn_nums.remove(0);
        }
        spoken_nums.entry(response).or_insert(vec![]).push(turn_num);
        last_num = response;
        turn_num += 1;
    }
    last_num
}
