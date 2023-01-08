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
        let template: Vec<char> = segments[0].chars().collect();
        let propogations: Vec<Propogation> = segments[1].lines().map(|l| l.parse().unwrap()).collect();
        let mut map = HashMap::new();
        for propogation in propogations {
            map.insert(propogation.pair, propogation.create);
        }
        let steps: usize = args[2].parse().unwrap();
        let counts = propogate(&template, &map, steps);
        println!("Length after step {}: {}", steps, counts.values().sum::<usize>());
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

fn propogate(existing: &Vec<char>, propogations: &HashMap<(char, char), char>, steps: usize) -> HashMap<char, usize> {
    let mut previous = existing[0];
    let mut counts = HashMap::new();
    increment(&mut counts, previous);
    for i in 1..existing.len() {
        let next = existing[i];
        increment(&mut counts, next);
        inner(&mut counts, previous, next, propogations, steps);
        previous = next;
    }
    counts
}

fn inner(counts: &mut HashMap<char, usize>, previous: char, next: char, propogations: &HashMap<(char, char), char>, steps: usize) {
    if steps > 0 {
        if let Some(&create) = propogations.get(&(previous, next)) {
            inner(counts, previous, create, propogations, steps - 1);
            increment(counts, create);
            inner(counts, create, next, propogations, steps - 1);
        } else {
            // nothing gets added here
        }
    }
}

fn increment(counts: &mut HashMap<char, usize>, key: char) {
    if let Some(existing) = counts.get(&key) {
        counts.insert(key, existing + 1);
    } else {
        counts.insert(key, 1);
    }
}