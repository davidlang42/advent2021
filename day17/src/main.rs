use std::env;
use std::fs;

struct Point {
    x: isize,
    y: isize
}

struct Probe {
    position: Point,
    velocity: Point
}

struct Area {
    min: Point,
    max: Point
}

enum ProbeResult {
    MissedShort,
    MissedLong,
    Hit { max_height: isize }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let target = parse_target_area(&text);
        let mut vy = target.min.y;
        let mut count = 0;
        loop {
            for vx in 0..(target.max.x+1) {
                let mut probe = Probe::new(vx, vy);
                if let ProbeResult::Hit { max_height } = probe.fire(&target) {
                    count += 1;
                    println!("HIT with a max height of {}, starting with ({}, {}), total count of {}", max_height, vx, vy, count);
                }
            }
            vy += 1;
        }
    } else {
        println!("Please provide 1 argument: Filename");
    }
}

impl Probe {
    fn new(x_velocity: isize, y_velocity: isize) -> Self {
        Self {
            position: Point {
                x: 0,
                y: 0
            },
            velocity: Point {
                x: x_velocity,
                y: y_velocity
            }
        }
    }

    fn step(&mut self) {
        self.position.x += self.velocity.x;
        self.position.y += self.velocity.y;
        self.velocity.x -= self.velocity.x.signum();
        self.velocity.y -= 1;
    }

    fn fire(&mut self, target: &Area) -> ProbeResult {
        let mut max_height = self.position.y;
        loop {
            self.step();
            if self.position.y > max_height {
                max_height = self.position.y;
            }
            if target.contains(&self.position) {
                return ProbeResult::Hit { max_height };
            }
            if self.position.x > target.max.x {
                return ProbeResult::MissedLong;
            }
            if self.position.y < target.min.y {
                return ProbeResult::MissedShort;
            }
        }
    }
}

impl Area {
    fn contains(&self, point: &Point) -> bool {
        point.x >= self.min.x && point.x <= self.max.x
            && point.y >= self.min.y && point.y <= self.max.y
    }
}

fn parse_target_area(line: &str) -> Area {
    //target area: x=20..30, y=-10..-5
    let coordinates: Vec<&str> = line.split(": ").nth(1).unwrap().split(", ").collect();
    let x_range: Vec<isize> = coordinates[0].split("=").nth(1).unwrap().split("..").map(|n| n.parse().unwrap()).collect();
    let y_range: Vec<isize> = coordinates[1].split("=").nth(1).unwrap().split("..").map(|n| n.parse().unwrap()).collect();
    Area {
        min: Point {
            x: x_range[0],
            y: y_range[0]
        },
        max: Point {
            x: x_range[1],
            y: y_range[1]
        }
    }
}