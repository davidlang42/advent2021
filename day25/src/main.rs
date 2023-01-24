use std::env;
use std::fs;
use std::str::FromStr;
use std::fmt::Display;
use std::fmt::Formatter;
use std::collections::HashMap;

#[derive(Eq, PartialEq, Copy, Clone)]
enum Direction {
    Right,
    Down
}

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
struct Point {
    down: usize,
    right: usize
}

struct Simulation {
    cucumbers: HashMap<Point, Direction>,
    width: usize,
    height: usize
}

impl FromStr for Simulation {
    type Err = String;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let mut cucumbers = HashMap::new();
        let mut height = 0;
        let mut width = 0;
        for (down, line) in text.lines().enumerate() {
            for (right, c) in line.chars().enumerate() {
                match c {
                    '>' => cucumbers.insert(Point { right, down }, Direction::Right),
                    'v' => cucumbers.insert(Point { right, down }, Direction::Down),
                    _ => None
                };
                width = right + 1;
            }
            height = down + 1;
        }
        Ok(Self {
            cucumbers,
            width,
            height
        })
    }
}

impl Display for Simulation {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        for down in 0..self.height {
            for right in 0..self.width {
                write!(f, "{}", match self.cucumbers.get(&Point { right, down }) {
                    Some(Direction::Right) => '>',
                    Some(Direction::Down) => 'v',
                    None => '.'
                })?;
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let mut simulation: Simulation = text.parse().unwrap();
        let mut step = 1;
        while simulation.step() {
            step += 1;
        }
        println!("Sea cucumbers stopped moving after {} steps", step);
    } else {
        println!("Please provide 1 argument: Filename");
    }
}

impl Simulation {
    fn step(&mut self) -> bool {
        let right = self.half_step(&Direction::Right);
        let down = self.half_step(&Direction::Down);
        right || down
    }

    fn half_step(&mut self, direction: &Direction) -> bool {
        let mut new_cucumbers = HashMap::new();
        let mut changes = false;
        for (p, d) in &self.cucumbers {
            if d == direction {
                let adjacent = self.adjacent_to(p, d);
                if !self.cucumbers.contains_key(&adjacent) {
                    new_cucumbers.insert(adjacent, *d);
                    changes = true;
                } else {
                    new_cucumbers.insert(*p, *d);
                }
            } else {
                new_cucumbers.insert(*p, *d);
            }
        }
        self.cucumbers = new_cucumbers;
        changes
    }

    fn adjacent_to(&self, point: &Point, direction: &Direction) -> Point {
        match direction {
            Direction::Right => {
                if point.right == self.width - 1 {
                    Point {
                        right: 0,
                        down: point.down
                    }
                } else {
                    Point {
                        right: point.right + 1,
                        down: point.down
                    }
                }
            },
            Direction::Down => {
                if point.down == self.height - 1 {
                    Point {
                        right: point.right,
                        down: 0
                    }
                } else {
                    Point {
                        right: point.right,
                        down: point.down + 1
                    }
                }
            }
        }
    }
}