use std::fs::File;
use std::io::Read;

fn main() -> () {
    let mut input = String::new();
    File::open("input.txt")
        .expect("Could not find input file")
        .read_to_string(&mut input)
        .expect("could not read input file");

    let mut puzzle_map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    part1(&mut puzzle_map);
    part2(&mut puzzle_map);
}

fn part1(puzzle_map: &mut Vec<Vec<char>>) {
    let count = do_puzzle(puzzle_map, 3, 1);

    println!("{}", count)
}

fn part2(puzzle_map: &mut Vec<Vec<char>>) {
    let product = do_puzzle(puzzle_map, 1, 1)
        * do_puzzle(puzzle_map, 3, 1)
        * do_puzzle(puzzle_map, 5, 1)
        * do_puzzle(puzzle_map, 7, 1)
        * do_puzzle(puzzle_map, 1, 2);

    println!("{}", product)
}

fn do_puzzle(puzzle_map: &mut Vec<Vec<char>>, x_step: usize, y_step: usize) -> usize {
    if puzzle_map.len() == 0 && puzzle_map[0].len() == 0 {
        return 0;
    }
    let (max_x, max_y) = (puzzle_map[0].len(), puzzle_map.len());
    let (mut x, mut count) = (0, 0);

    for y in (y_step..max_y).step_by(y_step) {
        x = (x + x_step).rem_euclid(max_x);
        match puzzle_map[y][x] {
            '#' => count += 1,
            _ => (),
        }
    }

    count
}
