use std::env;
use std::fs;
use std::str::FromStr;

struct Entry {
    unique: [Signal; 10],
    output: [Signal; 4]
}

#[derive(Debug, PartialEq, Eq)]
struct Signal([bool; 7]);

struct WireMap([PartialMap; 7]);

#[derive(Copy, Clone)]
enum PartialMap {
    Definite(usize),
    CouldBe([bool; 7])
}

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
            let mut wire_map = WireMap::new(); //TODO &entry.unique, &digits);
            for unique in &entry.unique {
                let easy_match = match unique.segments().len() {
                    2 => Some(&digits[1]), // "1"
                    4 => Some(&digits[4]), // "4"
                    3 => Some(&digits[7]), // "7"
                    7 => Some(&digits[8]), // "8"
                    _ => None
                };
                if let Some(actual) = easy_match {
                    wire_map.map(unique, actual);
                }
            } 
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
            if let PartialMap::Definite(new_i) = map.0[i] {
                signal[i] = self.0[new_i]; //TODO this might be backwards?
            } else {
                panic!("Cannot decode with an incomplete map");
            }
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

    // fn could_be(&self, map: &WireMap) -> bool {

    // }
}

impl WireMap {
    fn new() -> Self {
        WireMap([PartialMap::CouldBe([true; 7]); 7])
    }

    fn map(&mut self, input_signal: &Signal, output_signal: &Signal) {
        let mut inputs = input_signal.segments();
        let mut outputs = output_signal.segments();
        if inputs.len() != outputs.len() {
            panic!("Cannot map signals with different segment counts");
        }
        // remove any already mapped inputs and their respective output
        let mut i = 0;
        while i < inputs.len() {
            let input = inputs[i];
            if let PartialMap::Definite(output) = self.0[input] {
                if let Some(pos) = outputs.iter().position(|o| *o == output) {
                    outputs.remove(pos);
                    inputs.remove(i);
                } else {
                    panic!("Invalid mapping");
                }
            } else {
                i += 1;
            }
        }
        // mark each remaining input as only possibly mapping to any of the remaining outputs
        for input in inputs {
            if let PartialMap::CouldBe(old) = self.0[input] {
                let mut new = [false; 7];
                let mut remaining = Vec::new();
                for output in &outputs {
                    if old[*output] {
                        // this 'input' could still map to this 'output'
                        new[*output] = true;
                        remaining.push(output);
                    }
                }
                self.0[input] = if remaining.len() == 0 {
                    panic!("No options remaining");
                } else if remaining.len() == 1 {
                    PartialMap::Definite(*remaining[0])
                } else {
                    PartialMap::CouldBe(new)
                };
            } else {
                panic!("Definite found while updating CouldBe's");
            }
        }
    }
}