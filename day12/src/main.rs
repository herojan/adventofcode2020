use std::error::Error;
use std::fs::read_to_string;
use std::ops::Sub;
use std::str::FromStr;

fn main() {
    let input = read_to_string("input.txt").expect("Couldn't load input");
    part1(&input);
    part2(&input)
}

fn part1(input: &str) {
    let actions: Vec<Action> = input.lines().map(|l| l.parse().unwrap()).collect();
    let mut ship = Ship::new(Direction::E);

    for action in &actions {
        ship.act(action)
    }

    let manhattan = ship.position.manhattan_distance_to(&Point::default());
    println!("{:?}", manhattan)
}

fn part2(input: &str) {
    let actions: Vec<Action> = input.lines().map(|l| l.parse().unwrap()).collect();
    let mut ship = Ship::new(Direction::E);
    ship.waypoint.x += 10;
    ship.waypoint.y -= 1;

    for action in &actions {
        ship.act_waypoint(action)
    }

    let manhattan = ship.position.manhattan_distance_to(&Point::default());
    println!("{:?}", manhattan)
}

#[derive(Debug)]
struct Ship {
    position: Point,
    direction: Direction,
    waypoint: Point,
}

impl Ship {
    pub fn new(start_direction: Direction) -> Ship {
        Ship {
            direction: start_direction,
            position: Point::default(),
            waypoint: Point::default(),
        }
    }

    fn act(&mut self, action: &Action) {
        match action {
            Action::N(_) | Action::S(_) | Action::E(_) | Action::W(_) => self.position.act(action),
            Action::L(_) | Action::R(_) => self.direction.rotate(&action),
            Action::F(dist) => match self.direction {
                Direction::N => self.position.act(&Action::N(*dist)),
                Direction::S => self.position.act(&Action::S(*dist)),
                Direction::E => self.position.act(&Action::E(*dist)),
                Direction::W => self.position.act(&Action::W(*dist)),
            },
        }
    }

    fn act_waypoint(&mut self, action: &Action) {
        match action {
            Action::N(_) | Action::S(_) | Action::E(_) | Action::W(_) => self.waypoint.act(action),
            Action::L(_) | Action::R(_) => self.waypoint.rotate_around(&self.position, action),
            Action::F(times) => {
                let vector = self.waypoint - self.position;
                for _ in 0..*times {
                    self.position.translate(vector.x, vector.y);
                    self.waypoint.translate(vector.x, vector.y)
                }
            }
        }
    }
}

#[derive(Debug, Default, Copy, Clone)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn act(&mut self, action: &Action) {
        match action {
            Action::N(dist) => self.y -= dist,
            Action::S(dist) => self.y += dist,
            Action::E(dist) => self.x += dist,
            Action::W(dist) => self.x -= dist,
            _ => panic!("Translation should be N,S,E or W"),
        };
    }

    fn translate(&mut self, x: isize, y: isize) {
        self.x += x;
        self.y += y;
    }

    fn move_to(&mut self, direction: &Direction, amount: isize) {
        match direction {
            Direction::N => self.y += amount.abs() * -1,
            Direction::S => self.y += amount.abs(),
            Direction::E => self.x += amount.abs(),
            Direction::W => self.x += amount.abs() * -1,
        }
    }

    fn rotate_around(&mut self, other: &Point, action: &Action) {
        let diff = *self - *other;
        let mut relative_to_other = Point::default();
        let mut x_direction = if diff.x < 0 {
            Direction::W
        } else {
            Direction::E
        };
        let mut y_direction = if diff.y < 0 {
            Direction::N
        } else {
            Direction::S
        };
        x_direction.rotate(&action);
        y_direction.rotate(&action);

        relative_to_other.move_to(&x_direction, diff.x);
        relative_to_other.move_to(&y_direction, diff.y);

        self.x = other.x + relative_to_other.x;
        self.y = other.y + relative_to_other.y;
    }

    fn manhattan_distance_to(&self, other: &Point) -> usize {
        ((self.x + other.x).abs() + (self.y + other.y).abs()) as usize
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

#[derive(Debug, Clone)]
enum Direction {
    N,
    S,
    E,
    W,
}

impl Direction {
    fn rotate(&mut self, action: &Action) {
        let mut curr_degrees = match self {
            Direction::N => 0,
            Direction::S => 180,
            Direction::E => 90,
            Direction::W => 270,
        };

        curr_degrees = match action {
            Action::L(degrees) => (curr_degrees - degrees).rem_euclid(360),
            Action::R(degrees) => (curr_degrees + degrees).rem_euclid(360),
            _ => panic!("Rotation should be L or R"),
        };

        *self = match curr_degrees {
            0 => Direction::N,
            180 => Direction::S,
            90 => Direction::E,
            270 => Direction::W,
            _ => panic!(format!("Not allowed to have degrees of {}", curr_degrees)),
        };
    }
}

#[derive(Debug)]
enum Action {
    N(isize),
    S(isize),
    E(isize),
    W(isize),
    L(isize),
    R(isize),
    F(isize),
}

impl FromStr for Action {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let char = chars.next().unwrap();
        let num: String = chars.collect();
        let num: isize = num.parse().unwrap();
        match char {
            'N' => Ok(Action::N(num)),
            'S' => Ok(Action::S(num)),
            'E' => Ok(Action::E(num)),
            'W' => Ok(Action::W(num)),
            'L' => Ok(Action::L(num)),
            'R' => Ok(Action::R(num)),
            'F' => Ok(Action::F(num)),
            _ => Err(Box::from(format!("Invalid action {}", s))),
        }
    }
}
