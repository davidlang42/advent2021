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

impl Room {
    fn complete(&self) -> bool {
        for slot in self.slots {
            match slot {
                Some(amphipod) => if amphipod != self.required {
                    return false;
                },
                None => return false
            }
        }
        true
    }

    fn valid(&self) -> bool {
        for slot in self.slots {
            match slot {
                Some(amphipod) => if amphipod != self.required {
                    return false;
                },
                None => {}
            }
        }
        true
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
                if !room.complete() {
                    return false;
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
        let mut moves = Vec::new();
        // PROVIDED: Amphipods will never stop on the space immediately outside any room. They can move into that space so long as they immediately continue moving.
        for h in 0..self.hallway.len() {
            for r in 0..self.rooms.len() {
                if let (Some(_amphipod_in_hallway), Some(_into_room)) = (self.hallway[h], self.rooms[r]) {
                    if let Some(valid_move) = self.enter_room(h, r) {
                        moves.push(valid_move);
                    }
                }
            }
        }
        // PROVIDED: If an amphipod's starting room is not its destination room, it can stay in that room until it leaves the room.
        for r in 0..self.rooms.len() {
            if let Some(room) = self.rooms[r] {
                for s in 0..room.slots.len() {
                    if let Some(_amphipod_in_room) = room.slots[s] {
                        let mut possible_exits = self.exit_room(r, s);
                        moves.append(&mut possible_exits);
                        break; // nothing below this can move
                    }
                }
            }
        }
        moves
    }

    // PROVIDED: Amphipods will never move from the hallway into a room unless that room is their destination room and that room contains no amphipods which do not also have that room as their own destination.
    // ASSUMED: If an amphipod is entering the correct room, it will move all the way in immediately, because it can't possibly help not to.
    fn enter_room(&self, from_hallway_index: usize, into_room_index: usize) -> Option<(State, usize)> {
        let amphipod_in_hallway = self.hallway[from_hallway_index].expect("invalid hall index");
        let into_room = self.rooms[into_room_index].expect("invalid room index");
        if into_room.required != amphipod_in_hallway || !into_room.valid() {
            return None; // not the right room, or room has wrong amphipods
        }
        let hallway_range = if from_hallway_index > into_room_index {
            into_room_index..from_hallway_index
        } else {
            (from_hallway_index + 1)..(into_room_index + 1)
        };
        for h in hallway_range {
            if self.hallway[h].is_some() {
                return None; // movement blocked
            }
        }
        let mut free_slot = None;
        for s in 0..into_room.slots.len() {
            match into_room.slots[s] {
                None => free_slot = Some(s),
                Some(_) => break
            }
        }
        if let Some(slot) = free_slot {
            let movements = from_hallway_index.abs_diff(into_room_index) + 1 + slot;
            let energy = amphipod_in_hallway.energy() * movements;
            let mut new_state = self.clone();
            *new_state.hallway.get_mut(from_hallway_index).unwrap() = None;
            *new_state.rooms.get_mut(into_room_index).unwrap().as_mut().unwrap().slots.get_mut(slot).unwrap() = Some(amphipod_in_hallway);
            Some((new_state, energy))
        } else {
            None // no free slot
        }
    }

    // PROVIDED: Once an amphipod stops moving in the hallway, it will stay in that spot until it can move into a room. (That is, once any amphipod starts moving, any other amphipods currently in the hallway are locked in place and will not move again until they can move fully into a room.)
    // THEREFORE: When an amphipod exits a room, you have a choice of where in the hallway it will stop (as long as thats not in front of any room)
    fn exit_room(&self, from_room_index: usize, from_slot_index: usize) -> Vec<(State, usize)> {
        let from_room = self.rooms[from_room_index].expect("invalid room index");
        let amphipod_in_room = from_room.slots[from_slot_index].expect("invalid slot index");
        if from_room.required == amphipod_in_room && from_room.valid() {
            return Vec::new(); // already in the correct room with only correct amphipods
        }
        for s in 0..from_slot_index {
            if from_room.slots[s].is_some() {
                return Vec::new(); // movement blocked
            }
        }
        let free_hallway_left = self.free_hallway(from_room_index, -1);
        let free_hallway_right = self.free_hallway(from_room_index, 1);
        let mut valid_moves = Vec::new();
        for free_hallway in [free_hallway_left, free_hallway_right].concat() {
            // PROVIDED: Amphipods will never stop on the space immediately outside any room. They can move into that space so long as they immediately continue moving.
            if self.rooms[free_hallway].is_none() {
                let movements = from_slot_index + 1 + from_room_index.abs_diff(free_hallway);
                let energy = amphipod_in_room.energy() * movements;
                let mut new_state = self.clone();
                *new_state.hallway.get_mut(free_hallway).unwrap() = Some(amphipod_in_room);
                *new_state.rooms.get_mut(from_room_index).unwrap().as_mut().unwrap().slots.get_mut(from_slot_index).unwrap() = None;
                valid_moves.push((new_state, energy));
            }
        }
        valid_moves
    }

    fn free_hallway(&self, starting: usize, delta: isize) -> Vec<usize> {
        let mut result = Vec::new();
        let mut h = starting as isize + delta; // intentionally skip the starting value
        while h >= 0 && (h as usize) < self.hallway.len() {
            if self.hallway[h as usize].is_some() {
                break; // hallway blocked
            } else {
                result.push(h as usize);
            }
            h += delta;
        }
        result
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
