use std::env;
use std::fs;
use std::str::FromStr;
use itertools::Itertools;
use std::collections::HashSet;

struct Entry {
    unique: [Signal; 10],
    output: [Signal; 4]
}

#[derive(Hash, Debug, PartialEq, Eq)]
struct Signal([bool; 7]);

struct WireMap([usize; 7]);

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let entries: Vec<Entry> = text.lines().map(|l| l.parse().unwrap()).collect();
        let count1478 = entries.iter().flat_map(|e| e.output.iter()).filter(|s| match s.segments().len() {
            2 => true, // "1"
            4 => true, // "4"
            3 => true, // "7"
            7 => true, // "8"
            _ => false
        }).count();
        println!("Simple digits: {}", count1478);
        let digits = [
            Signal::from_str("abcefg").unwrap(),
            Signal::from_str("cf").unwrap(),
            Signal::from_str("acdeg").unwrap(),
            Signal::from_str("acdfg").unwrap(),
            Signal::from_str("bdcf").unwrap(),
            Signal::from_str("abdfg").unwrap(),
            Signal::from_str("abdefg").unwrap(),
            Signal::from_str("acf").unwrap(),
            Signal::from_str("abcdefg").unwrap(),
            Signal::from_str("abcdfg").unwrap()
        ];
        let mut sum = 0;
        for entry in entries {
            let wire_map = WireMap::new(&entry.unique, &digits);
            let mut output = 0;
            for raw in &entry.output {
                let decoded = raw.decode(&wire_map);
                let number = digits.iter().position(|d| *d == decoded).unwrap();
                output *= 10;
                output += number;
            }
            println!("Output number: {}", output);
            sum += output;
        }
        println!("Sum of output numbers: {}", sum);
    } else {
        println!("Please provide 1 argument: Filename");
    }
}

impl FromStr for Entry {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = line.split(" | ").collect();
        if parts.len() != 2 {
            return Err(format!("Expected 2 parts: {}", line));
        }
        let unique: Vec<Signal> = parts[0].split(" ").map(|s| s.parse().unwrap()).collect();
        if unique.len() != 10 {
            return Err(format!("Expected 10 unique signals: {}", parts[0]));
        }
        let output: Vec<Signal> = parts[1].split(" ").map(|s| s.parse().unwrap()).collect();
        if output.len() != 4 {
            return Err(format!("Expected 4 output signals: {}", parts[1]));
        }
        Ok(Entry {
            unique: unique.try_into().unwrap(),
            output: output.try_into().unwrap()
        })
    }
}

impl FromStr for Signal {
    type Err = String;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let mut signal = [false; 7];
        for c in string.chars() {
            let n: u8 = c as u8 - 'a' as u8;
            if n < 7 {
                signal[n as usize] = true;
            } else {
                return Err(format!("Invalid char: {}", c));
            }
        }
        Ok(Signal(signal))
    }
}

impl Signal {
    fn decode(&self, map: &WireMap) -> Signal {
        let mut signal = [false; 7];
        for i in 0..7 {
            let pos = map.0.iter().position(|o| *o == i).unwrap();
            signal[i] = self.0[pos];
        }
        Signal(signal)
    }

    fn segments(&self) -> Vec<usize> {
        let mut seggs = Vec::new();
        for i in 0..7 {
            if self.0[i] {
                seggs.push(i);
            }
        }
        seggs
    }
}

impl WireMap {
    fn new<const N: usize>(input_signals: &[Signal; N], output_signals: &[Signal; N]) -> Self {
        for map in Self::enumerate() {
            let mut remaining_outputs: HashSet<&Signal> = output_signals.iter().collect();
            let mut valid = true;
            for input in input_signals {
                let output = input.decode(&map);
                if !remaining_outputs.remove(&output) {
                    valid = false;
                    break;
                }
            }
            if valid {
                return map;
            }
        }
        panic!("No valid map found");
    }

    fn enumerate() -> Vec<Self> {
        [0,1,2,3,4,5,6].iter().permutations(7).map(|m| WireMap(m.iter().map(|v| **v).collect::<Vec<usize>>().try_into().unwrap())).collect()
    }
}