use std::env;
use std::fs;
use std::str::FromStr;
use std::collections::HashMap;

struct Propogation {
    pair: (char, char),
    create: char
}

struct PropogationMap {
    map: HashMap<(char, char), char>,
    cached_inner: HashMap<(char, char, usize), HashMap<char, usize>>
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
        let mut map = PropogationMap::new(&propogations);
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
        PropogationMap {
            map,
            cached_inner: HashMap::new()
        }
    }
    
    fn propogate(&mut self, template: &Vec<char>, steps: usize) -> HashMap<char, usize> {
        let mut previous = template[0];
        let mut counts = HashMap::new();
        increment(&mut counts, previous, 1);
        for i in 1..template.len() {
            let next = template[i];
            increment(&mut counts, next, 1);
            combine(&mut counts, &self.inner(previous, next, steps));
            previous = next;
        }
        counts
    }

    fn inner(&mut self, previous: char, next: char, steps: usize) -> HashMap<char, usize> {
        if let Some(cached) = self.cached_inner.get(&(previous, next, steps)) {
            cached.clone()
        } else {
            let mut counts = HashMap::new();
            if steps > 0 {
                if let Some(&create) = self.map.get(&(previous, next)) {
                    combine(&mut counts, &self.inner(previous, create, steps - 1));
                    increment(&mut counts, create, 1);
                    combine(&mut counts, &self.inner(create, next, steps - 1));
                } else {
                    // nothing gets added here
                }
            }
            self.cached_inner.insert((previous, next, steps), counts.clone());
            counts
        }
    }
}

fn increment(counts: &mut HashMap<char, usize>, key: char, delta: usize) {
    if let Some(existing) = counts.get(&key) {
        counts.insert(key, existing + delta);
    } else {
        counts.insert(key, delta);
    }
}

fn combine(counts: &mut HashMap<char, usize>, with: &HashMap<char, usize>) {
    for (&k, &v) in with {
        increment(counts, k, v);
    }
}