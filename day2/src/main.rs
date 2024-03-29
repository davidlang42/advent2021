use std::str::FromStr;
use std::env;
use std::fs;

struct Instruction {
    direction: Direction,
    distance: u32
}

struct Location {
    horizontal: u32,
    depth: u32
}

struct LocationWithAim {
    horizontal: u32,
    depth: u32,
    aim: i32
}

enum Direction {
    Forward, Down, Up
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let instructions: Vec<Instruction> = text.split("\r\n").map(|s| s.parse()
            .expect(&format!("Error parsing instruction {}", s))).collect();
        let location = process_instructions(&instructions);
        println!("Horizontal {}, Depth {}, Multiply {}", location.horizontal, location.depth, location.horizontal * location.depth);
        let with_aim = process_instructions_with_aim(&instructions);
        println!("(with aim) Horizontal {}, Depth {}, Multiply {}", with_aim.horizontal, with_aim.depth, with_aim.horizontal * with_aim.depth);
    } else {
        println!("Please provide 1 argument: Filename");
    }
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let words: Vec<&str> = line.split(" ").collect();
        if words.len() != 2 {
            Err(format!("Must be 2 words: {}", line))
        } else {
            let direction: Direction = match words[0] {
                "forward" => Ok(Direction::Forward),
                "up" => Ok(Direction::Up),
                "down" => Ok(Direction::Down),
                _ => Err(format!("Invalid direction: {}", words[0]))
            }.unwrap();
            let distance: u32 = words[1].parse().unwrap();
            Ok(Instruction { direction, distance })
        }
    }
}

fn process_instructions(instructions: &Vec<Instruction>) -> Location {
    let mut location = Location { horizontal: 0, depth: 0 };
    for instruction in instructions.iter() {
        match instruction.direction {
            Direction::Down => location.depth += instruction.distance,
            Direction::Up => location.depth -= instruction.distance,
            Direction::Forward => location.horizontal += instruction.distance,
        }
    }
    location
}

fn process_instructions_with_aim(instructions: &Vec<Instruction>) -> LocationWithAim {
    let mut location = LocationWithAim { horizontal: 0, depth: 0, aim: 0 };
    for instruction in instructions.iter() {
        match instruction.direction {
            Direction::Down => location.aim += instruction.distance as i32,
            Direction::Up => location.aim -= instruction.distance as i32,
            Direction::Forward => {
                location.horizontal += instruction.distance;
                location.depth = (location.depth as i32 + instruction.distance as i32 * location.aim) as u32;
            }
        }
    }
    location
}