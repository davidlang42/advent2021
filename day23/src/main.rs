use std::env;
use std::fs;
use std::str::FromStr;
use std::fmt::Display;
use std::fmt::Formatter;
use pathfinding::prelude::bfs;

#[derive(Hash, Eq, PartialEq, Clone, Copy)]
enum Amphipod {
    Amber,
    Bronze,
    Copper,
    Desert
}

#[derive(Hash, Eq, PartialEq, Clone, Copy)]
struct Room {
    required: Amphipod,
    slots: [Option<Amphipod>; 2]
}

#[derive(Hash, Eq, PartialEq, Clone)]
struct State {
    hallway: [Option<Amphipod>; 11],
    rooms: [Option<Room>; 11]
}

impl Amphipod {
    fn energy(&self) -> usize {
        match self {
            Amphipod::Amber => 1,
            Amphipod::Bronze => 10,
            Amphipod::Copper => 100,
            Amphipod::Desert => 1000
        }
    }

    fn from_usize(u: usize) -> Result<Self, <Self as FromStr>::Err> {
        match u {
            0 => Ok(Amphipod::Amber),
            1 => Ok(Amphipod::Bronze),
            2 => Ok(Amphipod::Copper),
            3 => Ok(Amphipod::Desert),
            _ => Err(format!("Invalid Amphipod: {}", u))
        }
    }

    fn from_char(c: char) -> Result<Self, <Self as FromStr>::Err> {
        match c {
            'A' => Ok(Amphipod::Amber),
            'B' => Ok(Amphipod::Bronze),
            'C' => Ok(Amphipod::Copper),
            'D' => Ok(Amphipod::Desert),
            _ => Err(format!("Invalid Amphipod: {}", c))
        }
    }

    fn to_char(&self) -> char {
        match self {
            Amphipod::Amber => 'A',
            Amphipod::Bronze => 'B',
            Amphipod::Copper => 'C',
            Amphipod::Desert => 'D'
        }
    }
}

impl FromStr for Amphipod {
    type Err = String;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let chars: Vec<char> = text.chars().collect();
        if chars.len() == 1 {
            Self::from_char(chars[0])
        } else {
            Err(format!("Expected 1 char: {}", text))
        }
    }
}

impl Display for Amphipod {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.to_char())
    }
}

impl FromStr for State {
    type Err = String;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = text.lines().collect();
        if lines.len() == 5 {
            if lines[2].len() < 11 {
                Err(format!("Expected at least 11 chars in 3rd line: {}", lines[2]))
            } else if lines[3].len() < 11 {
                Err(format!("Expected at least 11 chars in 4th line: {}", lines[3]))
            } else {
                let mut rooms = [None; 11];
                for amphipod_index in 0..4 {
                    let room_index = 2 * amphipod_index + 2; // 2,4,6,8
                    let char_index = room_index + 1; //3,5,7,9
                    rooms[room_index] = Some(Room {
                        required: Amphipod::from_usize(amphipod_index)?,
                        slots: [
                            Some(Amphipod::from_char(lines[2].chars().nth(char_index).unwrap())?),
                            Some(Amphipod::from_char(lines[3].chars().nth(char_index).unwrap())?),
                        ]
                    });
                }
                Ok(Self {
                    hallway: [None; 11],
                    rooms
                })
            }
        } else {
            Err(format!("Expected 5 lines: {}", text))
        }
    }
}

impl Display for State {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        writeln!(f, "#############")?;
        writeln!(f, "#{}#", self.hallway.iter().map(|h| match h {
            Some(a) => a.to_char(),
            None => '.'
        }).collect::<String>())?;
        writeln!(f, "#{}#", self.rooms.iter().map(|r| match r {
            Some(r) => match r.slots[0] {
                Some(a) => a.to_char(),
                None => '.'
            },
            None => '#'
        }).collect::<String>())?;
        writeln!(f, "#{}#", self.rooms.iter().map(|r| match r {
            Some(r) => match r.slots[1] {
                Some(a) => a.to_char(),
                None => '.'
            },
            None => '#'
        }).collect::<String>())?;
        write!(f, "  #########")
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let state: State = text.parse().unwrap();
        println!("{}", state);
    } else {
        println!("Please provide 1 argument: Filename");
    }
}
