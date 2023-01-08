use std::env;
use std::fs;
use std::str::FromStr;
use std::collections::HashMap;

struct Propogation {
    pair: (char, char),
    create: char
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 3 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let segments: Vec<&str> = text.split("\r\n\r\n").collect();
        let mut polymer: Vec<char> = segments[0].chars().collect();
        let propogations: Vec<Propogation> = segments[1].lines().map(|l| l.parse().unwrap()).collect();
        let mut map = HashMap::new();
        for propogation in propogations {
            map.insert(propogation.pair, propogation.create);
        }
        let steps: usize = args[2].parse().unwrap();
        for i in 0..steps {
            propogate(&mut polymer, &map);
            println!("Length after step {}: {}", i+1, polymer.len());
        }
        let counts = analyse(&polymer);
        let min = counts.values().min().unwrap();
        let max = counts.values().max().unwrap();
        println!("{} - {} = {}", max, min, max-min);
    } else {
        println!("Please provide 1 argument: Filename, Steps");
    }
}

impl FromStr for Propogation {
    type Err = String;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        //CN -> C
        let chars: Vec<char> = text.chars().collect();
        if chars.len() == 7 {
            Ok(Propogation {
                pair: (chars[0], chars[1]),
                create: chars[6]
            })
        } else{
            Err(format!("Expected 7 chars: {}", text))
        }
    }
}

fn propogate(polymer: &mut Vec<char>, propogations: &HashMap<(char, char), char> ) {
    let mut previous = polymer[0];
    let mut i = 1;
    while i < polymer.len() {
        let next = polymer[i];
        if let Some(create) = propogations.get(&(previous, next)) {
            polymer.insert(i, *create);
            i += 2;
        } else {
            i += 1;
        }
        previous = next;
    }
}

fn analyse(polymer: &Vec<char>) -> HashMap<char, usize> {
    let mut counts = HashMap::new();
    for c in polymer {
        if let Some(existing) = counts.get(c) {
            counts.insert(*c, existing + 1);
        } else {
            counts.insert(*c, 1);
        }
    }
    counts
}