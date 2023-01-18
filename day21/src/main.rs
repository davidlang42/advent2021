use std::env;
use std::fs;
use std::str::FromStr;

struct Player {
    position: usize,
    score: usize
}

struct DeterministicDie {
    sides: usize,
    rolls: usize
}

impl DeterministicDie {
    fn new(sides: usize) -> Self {
        Self {
            sides,
            rolls: 0
        }
    }

    fn roll(&mut self) -> usize {
        self.rolls += 1;
        let value = self.rolls % self.sides;
        if value == 0 {
            self.sides
        } else {
            value
        }
    }
}

enum PlayResult {
    Winner(usize),
    None
}

const BOARD_SIZE: usize = 10;

impl Player {
    fn new(starting_position: usize) -> Self {
        Self {
            score: 0,
            position: starting_position
        }
    }

    fn play(&mut self, die: &mut DeterministicDie) -> PlayResult {
        self.advance(die.roll() + die.roll() + die.roll());
        self.score += self.position;
        if self.score >= 1000 {
            PlayResult::Winner(self.score)
        } else {
            PlayResult::None
        }
    }

    fn advance(&mut self, spaces: usize) {
        let new_pos = (self.position + spaces) % BOARD_SIZE;
        self.position = if new_pos == 0 {
            BOARD_SIZE
        } else {
            new_pos
        };
    }
}

impl FromStr for Player {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        Ok(Player::new(line.split(": ").nth(1).unwrap().parse().unwrap()))
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let mut players: Vec<Player> = text.lines().map(|l| l.parse().unwrap()).collect();
        let mut die = DeterministicDie::new(100);
        loop {
            for i in 0..players.len() {
                if let PlayResult::Winner(score) = players[i].play(&mut die) {
                    println!("Player {} wins with {} points", i+1, score);
                    let lowest = players.iter().map(|p| p.score).min().unwrap();
                    println!("Lowest score * Rolls = {} * {} = {}", lowest, die.rolls, lowest * die.rolls);
                    return;
                }
            }
        }
    } else {
        println!("Please provide 1 argument: Filename");
    }
}
