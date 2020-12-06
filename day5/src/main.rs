use std::cmp::Ordering;
use std::error::Error;
use std::fs::read_to_string;
use std::str::FromStr;

fn main() {
    let input = read_to_string("input.txt").expect("could not read input file");
    let mut seats: Vec<Seat> = input.lines().map(|l| l.parse().unwrap()).collect();
    part1(&seats);
    part2(&mut seats);
}

fn part1(seats: &Vec<Seat>) {
    println!("{}", seats.iter().max().unwrap().id)
}

fn part2(seats: &mut Vec<Seat>) {
    seats.sort();
    let mut prev = seats.first().unwrap().id;
    for seat in seats {
        let seat_id = seat.id;
        if seat_id - prev > 1 {
            println!("{}", seat_id - 1);
            break;
        } else {
            prev = seat_id
        }
    }
}

#[derive(Hash, Eq, PartialEq, Debug, Default)]
struct Seat {
    row: u8,
    column: u8,
    id: u16,
}

impl PartialOrd for Seat {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Seat {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}

impl FromStr for Seat {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut row_range = 0..128;
        let mut col_range = 0..8;
        for char in s.chars() {
            let mid_row = (&row_range.start + &row_range.end) / 2;
            let mid_col = (&col_range.start + &col_range.end) / 2;
            match char {
                'F' => (&mut row_range).end = mid_row,
                'B' => (&mut row_range).start = mid_row,
                'L' => (&mut col_range).end = mid_col,
                'R' => (&mut col_range).start = mid_col,
                _ => panic!("Invalid char"),
            }
        }

        Ok(Seat {
            row: row_range.start,
            column: col_range.start,
            id: row_range.start as u16 * 8 + col_range.start as u16,
        })
    }
}
