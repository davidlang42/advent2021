use std::env;
use std::fs;
use std::str::FromStr;
use std::collections::HashMap;

#[derive(Clone, Hash, Eq, PartialEq)]
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

struct DiracDie {
    sides: usize
}

impl DiracDie {
    fn new(sides: usize) -> Self {
        Self {
            sides
        }
    }

    fn roll(&self) -> Vec<usize> {
        (1..(self.sides+1)).collect()
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

    fn play_dirac(&self, die: &DiracDie) -> Vec<Player> {
        let mut result = Vec::new();
        for d1 in die.roll() {
            for d2 in die.roll() {
                for d3 in die.roll() {
                    let mut copy = self.clone();
                    copy.advance(d1 + d2 + d3);
                    copy.score += copy.position;
                    result.push(copy);
                }
            }
        }
        result
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
        run_deterministic(players);
        players = text.lines().map(|l| l.parse().unwrap()).collect();
        run_dirac(players);
    } else {
        println!("Please provide 1 argument: Filename");
    }
}

fn run_deterministic(mut players: Vec<Player>) {
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
}

fn run_dirac(players: Vec<Player>) {
    let die = DiracDie::new(3);
    let universe = Universe::new(players);
    let wins = universe.simulate(&die);
    for (i, wins) in wins.0.iter() {
        println!("Player {} wins in {} universes", i+1, wins);
    }
    println!("Most wins: {}", wins.most());
}

struct Wins(HashMap<usize, usize>);

impl Wins {
    fn new() -> Self {
        Wins(HashMap::new())
    }

    fn increment(&mut self, winner: usize, count: usize) {
        match self.0.get(&winner) {
            Some(existing) => self.0.insert(winner, existing + count),
            None => self.0.insert(winner, count)
        };
    }

    fn most(&self) -> usize {
        *self.0.values().max().unwrap()
    }
}

#[derive(Eq, Hash, PartialEq, Clone)]
struct Universe {
    players: Vec<Player>,
    next_player: usize
}

struct Multiverse {
    universes: HashMap<Universe, usize>
}

impl Multiverse {
    fn new() -> Self {
        Self {
            universes: HashMap::new()
        }
    }

    fn push(&mut self, universe: Universe, count: usize) {
        match self.universes.get(&universe) {
            Some(existing) => self.universes.insert(universe, existing + count),
            None => self.universes.insert(universe, count)
        };
    }

    fn pop(&mut self) -> (Universe, usize) {
        let key = self.universes.keys().next().unwrap();
        let uni = key.clone();
        let count = self.universes.remove(&uni).unwrap();
        (uni, count)
    }

    fn len(&self) -> usize {
        self.universes.len()
    }
}

impl Universe {
    fn new(players: Vec<Player>) -> Self {
        Self {
            players,
            next_player: 0
        }
    }

    fn simulate(self, die: &DiracDie) -> Wins {
        let mut multi = Multiverse::new();
        multi.push(self, 1);
        let mut wins = Wins::new();
        while multi.len() > 0 {
            let (uni, count) = multi.pop();
            for new_player in uni.players[uni.next_player].play_dirac(die) {
                if new_player.score >= 21 {
                    wins.increment(uni.next_player, count);
                } else {
                    let new_uni = uni.split(new_player);
                    multi.push(new_uni, count);
                }
            }
        }
        wins
    }

    fn split(&self, replacement_player: Player) -> Universe {
        let mut new_players = self.players.clone();
        new_players[self.next_player] = replacement_player;
        let new_next_player = (self.next_player + 1) % self.players.len();
        Self {
            players: new_players,
            next_player: new_next_player
        }
    }
}