use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fs::read_to_string;
use std::str::FromStr;

fn main() {
    let input = read_to_string("input.txt").expect("Couldn't find input file");

    part1(&input);
    part2(&input)
}

fn part1(input: &str) {
    solve(input, 1, 4)
}

fn part2(input: &str) {
    // just have a massive vision limit...
    solve(input, isize::MAX as usize, 5)
}

fn solve(input: &str, vision_limit: usize, max_occupied_seats: usize) {
    let mut current_floor_plan: FloorPlan = input.parse().unwrap();
    let mut i = 0;
    loop {
        let original = current_floor_plan.clone();
        let new_plan = current_floor_plan.seat_the_peeps(vision_limit, max_occupied_seats);
        if original == new_plan || i == 10000 {
            break;
        }
        current_floor_plan = new_plan;
        i += 1;
    }
    let mut occupied_count = 0;
    for states in current_floor_plan.seat_rows {
        for state in states {
            match state {
                SeatState::OCCUPIED => occupied_count += 1,
                _ => (),
            }
        }
    }
    println!("{}", occupied_count)
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct FloorPlan {
    seat_rows: Vec<Vec<SeatState>>,
    view_directions: Vec<(isize, isize)>,
}

impl FloorPlan {
    pub fn new(seat_rows: Vec<Vec<SeatState>>) -> FloorPlan {
        FloorPlan {
            seat_rows,
            view_directions: vec![
                (-1, -1),
                (-1, 0),
                (-1, 1),
                (0, 1),
                (1, 1),
                (1, 0),
                (1, -1),
                (0, -1),
            ],
        }
    }
    fn occupied_neighbour_count(&self, i: usize, j: usize, vision_limit: usize) -> usize {
        let mut occupied_count = 0;
        let i = i as isize;
        let j = j as isize;
        let vision_limit = vision_limit as isize;
        for (vi, vj) in &self.view_directions {
            for vl in 1..=vision_limit {
                let i_index = i + vi * vl;
                let j_index = j + vj * vl;
                if i_index >= 0 && j_index >= 0 {
                    let maybe_state = self
                        .seat_rows
                        .get(i_index as usize)
                        .and_then(|v| v.get(j_index as usize));
                    if let Some(state) = maybe_state {
                        match state {
                            SeatState::OCCUPIED => {
                                occupied_count += 1;
                                break;
                            }
                            SeatState::EMPTY => {
                                break;
                            }
                            _ => (),
                        }
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }
        }

        occupied_count
    }

    fn seat_the_peeps(&self, vision_limit: usize, max_occupied_seats: usize) -> FloorPlan {
        let mut new_plan = vec![];
        for i in 0..self.seat_rows.len() {
            new_plan.push(vec![]);
            let states = &self.seat_rows[i];
            for j in 0..states.len() {
                let state = &states[j];
                let occupied_neighbours = self.occupied_neighbour_count(i, j, vision_limit);
                let new_states = new_plan.last_mut().unwrap();
                match state {
                    SeatState::EMPTY => {
                        if occupied_neighbours == 0 {
                            new_states.push(SeatState::OCCUPIED)
                        } else {
                            new_states.push(*state)
                        }
                    }
                    SeatState::OCCUPIED => {
                        if occupied_neighbours >= max_occupied_seats {
                            new_plan.last_mut().unwrap().push(SeatState::EMPTY)
                        } else {
                            new_states.push(*state)
                        }
                    }
                    _ => new_states.push(*state),
                }
            }
        }

        FloorPlan::new(new_plan)
    }
}

impl FromStr for FloorPlan {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let seat_rows: Vec<Vec<SeatState>> = s
            .lines()
            .map(|l| {
                l.chars()
                    .map(|c| match c {
                        '#' => SeatState::OCCUPIED,
                        '.' => SeatState::NA,
                        'L' => SeatState::EMPTY,
                        _ => panic!(format!("Unrecognised seat state {}", c)),
                    })
                    .collect()
            })
            .collect();

        Ok(FloorPlan::new(seat_rows))
    }
}

impl Display for FloorPlan {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for seat_row in &self.seat_rows {
            for seat in seat_row {
                let str = match seat {
                    SeatState::OCCUPIED => "#",
                    SeatState::EMPTY => "L",
                    SeatState::NA => ".",
                };
                write!(f, "{}", str)?;
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum SeatState {
    OCCUPIED,
    EMPTY,
    NA,
}
