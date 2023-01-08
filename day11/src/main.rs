use std::env;
use std::fs;
use std::str::FromStr;

struct Octopus(u32);

struct Grid {
    octopi: Vec<Vec<Octopus>>,
    height: usize,
    width: usize
}

struct Flash {
    r: usize,
    c: usize
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 3 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let mut grid: Grid = text.parse().unwrap();
        let rounds: usize = args[2].parse().unwrap();
        let mut flashes = 0;
        for _ in 0..rounds {
            let f = grid.increment_all();
            flashes += f.len();
        }
        println!("{} flashes after {} rounds", flashes, rounds);
    } else {
        println!("Please provide 2 arguments: Filename, Rounds");
    }
}

impl FromStr for Grid {
    type Err = String;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let octopi: Vec<Vec<Octopus>> = text.lines().map(|l| l.chars().map(|c| Octopus(c.to_digit(10).unwrap())).collect()).collect();
        Ok(Self {
            height: octopi.len(),
            width: octopi[0].len(),
            octopi
        })
    }
}

impl Grid {
    fn increment_all(&mut self) -> Vec<Flash> {
        let mut flashes = Vec::new();
        for r in 0..self.height {
            for c in 0..self.width {
                flashes.append(&mut self.increment(r, c));
            }
        }
        for flash in &flashes {
            self.octopi[flash.r][flash.c].reset();
        }
        flashes
    }

    fn increment(&mut self, r_index: usize, c_index: usize) -> Vec<Flash> {
        let mut flashes = Vec::new();
        if self.octopi[r_index][c_index].increment() {
            flashes.push(Flash {
                r: r_index,
                c: c_index 
            });
            let r_min = if r_index == 0 { r_index } else { r_index - 1 };
            let r_max = if r_index == self.height - 1 { r_index } else { r_index + 1 };
            let c_min = if c_index == 0 { c_index } else { c_index - 1 };
            let c_max = if c_index == self.width - 1 { c_index } else { c_index + 1 };
            for r in r_min..(r_max+1) {
                for c in c_min..(c_max+1) {
                    if r != r_index || c != c_index {
                        flashes.append(&mut self.increment(r, c));
                    }
                }
            }
        }
        flashes
    }
}

impl Octopus {
    fn increment(&mut self) -> bool {
        self.0 += 1;
        self.0 == 10 // return true if flash
    }

    fn reset(&mut self) {
        self.0 = 0;
    }
}