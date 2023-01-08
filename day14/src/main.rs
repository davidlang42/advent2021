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
        polymer = propogate(&polymer, &map, steps);
        println!("Length after step {}: {}", steps, polymer.len());
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

fn propogate(existing: &Vec<char>, propogations: &HashMap<(char, char), char>, steps: usize) -> Vec<char> {
    let mut previous = existing[0];
    let mut new = vec![previous];
    for i in 1..existing.len() {
        let next = existing[i];
        new.append(&mut inner(previous, next, propogations, steps));
        new.push(next);
        previous = next;
    }
    new
}

fn inner(previous: char, next: char, propogations: &HashMap<(char, char), char>, steps: usize) -> Vec<char> {
    let mut new = Vec::new();
    if steps > 0 {
        if let Some(&create) = propogations.get(&(previous, next)) {
            new.append(&mut inner(previous, create, propogations, steps - 1));
            new.push(create);
            new.append(&mut inner(create, next, propogations, steps - 1));
        } else {
            // nothing gets added here
        }
    }
    new
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