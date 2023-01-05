use std::env;
use std::fs;
use std::str::FromStr;

struct Entry {
    unique: [Signal; 10],
    output: [Signal; 4]
}

#[derive(Debug, PartialEq, Eq)]
struct Signal([bool; 7]);

//struct WireMap([usize; 7]);

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let entries: Vec<Entry> = text.lines().map(|l| l.parse().unwrap()).collect();
        let count1478 = entries.iter().flat_map(|e| e.output.iter()).filter(|s| match s.segments() {
            2 => true, // "1"
            3 => true, // "4"
            4 => true, // "7"
            7 => true, // "8"
            _ => false
        }).count();
        println!("Simple digits: {}", count1478);
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
    // fn decode(&self, map: &WireMap) -> Signal {
    //     let mut signal = [false; 7];
    //     for i in 0..7 {
    //         signal[i] = self[map[i]];// this might be backwards?
    //     }
    //     Signal(signal)
    // }

    fn segments(&self) -> usize {
        let mut count = 0;
        for i in 0..7 {
            if self.0[i] {
                count += 1;
            }
        }
        count
    }
}