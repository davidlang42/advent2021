use std::env;
use std::fs;
use std::str::FromStr;
use std::fmt::Display;
use std::fmt::Formatter;
use pathfinding::prelude::astar;

#[derive(Hash, Eq, PartialEq, Clone, Copy)]
enum Amphipod {
    Amber,
    Bronze,
    Copper,
    Desert
}

const ROOM_SLOTS: usize = 2;
const HALLWAY_WIDTH: usize = 11;

#[derive(Hash, Eq, PartialEq, Clone, Copy)]
struct Room {
    required: Amphipod,
    slots: [Option<Amphipod>; ROOM_SLOTS]
}

#[derive(Hash, Eq, PartialEq, Clone)]
struct State {
    hallway: [Option<Amphipod>; HALLWAY_WIDTH],
    rooms: [Option<Room>; HALLWAY_WIDTH]
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

    fn will_enter(&self, room: &Room) -> bool {
        // PROVIDED: Amphipods will never move from the hallway into a room unless that room is their destination room and that room contains no amphipods which do not also have that room as their own destination.
        if room.required != *self {
            return false;
        }
        for slot in room.slots {
            if let Some(existing_amphipod) = slot {
                if existing_amphipod != *self {
                    return false;
                }
            }
        }
        room.slots[0].is_none()
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
        if lines.len() == 3 + ROOM_SLOTS  {
            for s in 0..ROOM_SLOTS {
                if lines[s+2].len() < HALLWAY_WIDTH {
                    return Err(format!("Expected at least {} chars in line {}: {}", HALLWAY_WIDTH, s+3, lines[s+2]))
                }
            }
            let mut rooms = [None; HALLWAY_WIDTH];
            for amphipod_index in 0..4 {
                let room_index = 2 * amphipod_index + 2; // 2,4,6,8
                let char_index = room_index + 1; //3,5,7,9
                let mut slots = [None; ROOM_SLOTS];
                for s in 0..ROOM_SLOTS {
                    slots[s] = Some(Amphipod::from_char(lines[s+2].chars().nth(char_index).unwrap())?);
                }
                rooms[room_index] = Some(Room {
                    required: Amphipod::from_usize(amphipod_index)?,
                    slots
                });
            }
            Ok(Self {
                hallway: [None; HALLWAY_WIDTH],
                rooms
            })
        } else {
            Err(format!("Expected {} lines: {}", 3 + ROOM_SLOTS, text))
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

impl State {
    fn complete(&self) -> bool {
        for hallway in self.hallway {
            if hallway.is_some() {
                return false;
            }
        }
        for option_room in self.rooms {
            if let Some(room) = option_room {
                for slot in room.slots {
                    match slot {
                        Some(amphipod) => if amphipod != room.required {
                            return false;
                        },
                        None => return false
                    }
                }
            }
        }
        true
    }

    fn minimum_cost_to_complete(&self) -> usize {
        let mut cost = 0;
        for h in 0..self.hallway.len() {
            if let Some(amphipod) = self.hallway[h] {
                let required_room_index = self.room_index_for(&amphipod);
                let minimum_moves = h.abs_diff(required_room_index) + 1;
                cost += minimum_moves * amphipod.energy();
            }
        }
        for r in 0..self.rooms.len() {
            if let Some(room) = self.rooms[r] {
                for s in 0..room.slots.len() {
                    if let Some(amphipod) = room.slots[s] {
                        if amphipod != room.required {
                            let required_room_index = self.room_index_for(&amphipod);
                            let minimum_moves = s + 1 + r.abs_diff(required_room_index) + 1;
                            cost += minimum_moves * amphipod.energy();
                        }
                    }
                }
            }
        }
        cost
    }

    fn room_index_for(&self, amphipod: &Amphipod) -> usize {
        for i in 0..self.rooms.len() {
            if let Some(room) = self.rooms[i] {
                if room.required == *amphipod {
                    return i;
                }
            }
        }
        panic!("Room not found for Amphipod: {}", amphipod);
    }

    fn possible_moves(&self) -> Vec<(State, usize)> {
        // ASSUMED: If an amphipod is in a slot of the correct room but the next slot down is free, it will move all the way in immediately, because it can't possibly help not to.
        for r in 0..self.rooms.len() {
            if let Some(room) = self.rooms[r] {
                for s in (0..room.slots.len()).rev() {
                    if let Some(amphipod_in_room) = room.slots[s] {
                        if amphipod_in_room == room.required {
                            if let Some(valid_move) = self.move_in_room(r, s, s + 1) {
                                return vec![valid_move];
                            }
                        } else {
                            break;
                        }
                    }
                }
            }
        }
        // PROVIDED: Amphipods will never stop on the space immediately outside any room. They can move into that space so long as they immediately continue moving.
        for h in 0..self.hallway.len() {
            if let (Some(amphipod_in_hallway), Some(outside_room)) = (self.hallway[h], self.rooms[h]) {
                if amphipod_in_hallway.will_enter(&outside_room) {
                    return vec![self.enter_room(h).unwrap()]; // ASSUMED: If an amphipod can move into its correct room, it will do so immediately, because it can't possibly help not to.
                } else {
                    let mut moves = Vec::new();
                    if let Some(valid_move) = self.move_in_hallway(h, h as isize - 1) {
                        moves.push(valid_move);
                    }
                    if let Some(valid_move) = self.move_in_hallway(h, h as isize + 1) {
                        moves.push(valid_move);
                    }
                    if moves.len() > 0 {
                        return moves;
                    } else {
                        //panic!("It is not allowed to stop on the space immediately outside a room");
                    }
                }
            }
        }
        // PROVIDED: Once an amphipod stops moving in the hallway, it will stay in that spot until it can move into a room. (That is, once any amphipod starts moving, any other amphipods currently in the hallway are locked in place and will not move again until they can move fully into a room.)
        //TODO not sure how to enforce this, but maybe it doesn't matter?
        let mut moves = Vec::new();
        for h in 0..self.hallway.len() {
            if let Some(_amphipod_in_hallway_not_outside_room) = self.hallway[h] {
                if let Some(valid_move) = self.move_in_hallway(h, h as isize - 1) {
                    moves.push(valid_move);
                }
                if let Some(valid_move) = self.move_in_hallway(h, h as isize + 1) {
                    moves.push(valid_move);
                }
            }
        }
        for r in 0..self.rooms.len() {
            if let Some(room) = self.rooms[r] {
                let mut wrong_amphipod_found = false;
                for s in (0..room.slots.len()).rev() {
                    if let Some(amphipod_in_room) = room.slots[s] {
                        if amphipod_in_room != room.required {
                            wrong_amphipod_found = true;
                        }
                    }
                    if wrong_amphipod_found {
                        if s == 0 {
                            if let Some(valid_move) = self.exit_room(r) {
                                moves.push(valid_move);
                            }
                        } else {
                            if let Some(valid_move) = self.move_in_room(r, s, s - 1) {
                                moves.push(valid_move);
                            }
                        }
                    }
                }
            }
        }
        moves
    }

    fn enter_room(&self, from_hallway_index: usize) -> Option<(State, usize)> {
        if from_hallway_index >= self.hallway.len() || self.hallway[from_hallway_index].is_none() || self.rooms[from_hallway_index].is_none() || self.rooms[from_hallway_index].unwrap().slots[0].is_some() {
            None
        } else {
            let mut hallway = self.hallway.clone();
            let mut rooms = self.rooms.clone();
            let amphipod_to_move = self.hallway[from_hallway_index];
            let energy = amphipod_to_move.unwrap().energy();
            let mut new_room = rooms[from_hallway_index].unwrap();
            new_room.slots[0] = amphipod_to_move;
            rooms[from_hallway_index] = Some(new_room);
            hallway[from_hallway_index] = None;
            Some((Self {
                hallway,
                rooms
            }, energy))
        }
    }

    fn exit_room(&self, from_room_index: usize) -> Option<(State, usize)> {
        if from_room_index >= self.rooms.len() || self.hallway[from_room_index].is_some() || self.rooms[from_room_index].is_none() || self.rooms[from_room_index].unwrap().slots[0].is_none() {
            None
        } else {
            let mut hallway = self.hallway.clone();
            let mut rooms = self.rooms.clone();
            let amphipod_to_move = self.rooms[from_room_index].unwrap().slots[0];
            let energy = amphipod_to_move.unwrap().energy();
            let mut new_room = rooms[from_room_index].unwrap();
            new_room.slots[0] = None;
            rooms[from_room_index] = Some(new_room);
            hallway[from_room_index] = amphipod_to_move;
            Some((Self {
                hallway,
                rooms
            }, energy))
        }
    }

    fn move_in_hallway(&self, from_hallway_index: usize, to_hallway_index: isize) -> Option<(State, usize)> {
        if from_hallway_index >= self.hallway.len() || to_hallway_index < 0 || to_hallway_index as usize >= self.hallway.len() || self.hallway[from_hallway_index].is_none() || self.hallway[to_hallway_index as usize].is_some() {
            None
        } else {
            let mut hallway = self.hallway.clone();
            let amphipod_to_move = self.hallway[from_hallway_index];
            let energy = amphipod_to_move.unwrap().energy();
            hallway[from_hallway_index] = None;
            hallway[to_hallway_index as usize] = amphipod_to_move;
            Some((Self {
                hallway,
                rooms: self.rooms.clone()
            }, energy))
        }
    }

    fn move_in_room(&self, room_index: usize, from_slot_index: usize, to_slot_index: usize) -> Option<(State, usize)> {
        if room_index >= self.rooms.len() || self.rooms[room_index].is_none() || from_slot_index >= self.rooms[room_index].unwrap().slots.len() || to_slot_index >= self.rooms[room_index].unwrap().slots.len() || self.rooms[room_index].unwrap().slots[from_slot_index].is_none() || self.rooms[room_index].unwrap().slots[to_slot_index].is_some() {
            None
        } else {
            let mut rooms = self.rooms.clone();
            let mut new_room = rooms[room_index].unwrap();
            let amphipod_to_move = new_room.slots[from_slot_index];
            let energy = amphipod_to_move.unwrap().energy();
            new_room.slots[from_slot_index] = None;
            new_room.slots[to_slot_index] = amphipod_to_move;
            rooms[room_index] = Some(new_room);
            Some((Self {
                rooms,
                hallway: self.hallway.clone()
            }, energy))
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let state: State = text.parse().unwrap();
        let (path, energy_cost) = astar(&state, |s| s.possible_moves(), |s| s.minimum_cost_to_complete(), |s| s.complete()).expect("no solution found");
        for s in &path {
            println!("{}\n", s);
        }
        println!("Completed in {} moves with a total energy cost of {}", path.len() - 1, energy_cost);
    } else {
        println!("Please provide 1 argument: Filename");
    }
}
