use std::env;
use std::fs;
use std::str::FromStr;
use std::collections::HashMap;

struct Propogation {
    pair: (char, char),
    create: char
}

struct PropogationMap(HashMap<(char, char), char>);

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 3 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let segments: Vec<&str> = text.split("\r\n\r\n").collect();
        let template: Vec<char> = segments[0].chars().collect();
        let propogations: Vec<Propogation> = segments[1].lines().map(|l| l.parse().unwrap()).collect();
        let map = PropogationMap::new(&propogations);
        let steps: usize = args[2].parse().unwrap();
        let counts = map.propogate(&template, steps);
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

impl PropogationMap {
    fn new(propogations: &Vec<Propogation>) -> Self {
        let mut map = HashMap::new();
        for propogation in propogations {
            map.insert(propogation.pair, propogation.create);
        }
        PropogationMap(map)
    }
    
    fn propogate(&self, template: &Vec<char>, steps: usize) -> HashMap<char, usize> {
        let mut previous = template[0];
        let mut counts = HashMap::new();
        increment(&mut counts, previous, 1);
        for i in 1..template.len() {
            let next = template[i];
            increment(&mut counts, next, 1);
            for (k, v) in self.inner(previous, next, steps) {
                increment(&mut counts, k, v);
            }
            previous = next;
        }
        counts
    }

    fn inner(&self, previous: char, next: char, steps: usize) -> HashMap<char, usize> {
        let mut counts = HashMap::new();
        if steps > 0 {
            if let Some(&create) = self.0.get(&(previous, next)) {
                for (k, v) in self.inner(previous, create, steps - 1) {
                    increment(&mut counts, k, v);
                }
                increment(&mut counts, create, 1);
                for (k, v) in self.inner(create, next, steps - 1) {
                    increment(&mut counts, k, v);
                }
            } else {
                // nothing gets added here
            }
        }
        counts
    }
}

fn increment(counts: &mut HashMap<char, usize>, key: char, delta: usize) {
    if let Some(existing) = counts.get(&key) {
        counts.insert(key, existing + delta);
    } else {
        counts.insert(key, delta);
    }
}