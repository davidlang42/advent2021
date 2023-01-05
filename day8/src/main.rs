use std::env;
use std::fs;
use std::str::FromStr;

struct Entry {
    unique: [Signal; 10],
    output: [Signal; 4]
}

#[derive(Debug, PartialEq, Eq)]
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
        let possible = [[true; 7]; 7]; // anythings possible
        let inputs: Vec<&Signal> = input_signals.iter().collect();
        let outputs: Vec<&Signal> = output_signals.iter().collect();
        Self::solve(possible, inputs, outputs).expect("No solution found")
    }

    fn solve(initial_possible: [[bool; 7]; 7], initial_inputs: Vec<&Signal>, initial_outputs: Vec<&Signal>) -> Option<WireMap> {
        let mut possible = initial_possible;
        let mut inputs: Vec<&Signal> = initial_inputs;
        let mut outputs: Vec<&Signal> = initial_outputs;
        while inputs.len() > 0 {
            let mut matched = false;
            for i in 0..inputs.len() {
                let input: &Signal = inputs[i];
                let possible_outputs: Vec<(usize, &&Signal)> = outputs.iter().enumerate().filter(|(_, output)| Self::could_be(&possible, input, output)).collect();
                match possible_outputs.len() {
                    0 => return None, // no possible matching output
                    1 => {
                        matched = true;
                        let (o, output) = possible_outputs[0];
                        //println!("Found match: {} with {}", input_signals.iter().position(|x| *x == *input).unwrap(), output_signals.iter().position(|x| *x == **output).unwrap());
                        Self::map_segments(&mut possible, input, output);
                        inputs.remove(i);
                        outputs.remove(o);
                        break;
                    },
                    _ => {}
                }
            }
            if !matched {
                let chosen_input = inputs[0];
                let possible_outputs: Vec<(usize, &&Signal)> = outputs.iter().enumerate().filter(|(_, output)| Self::could_be(&possible, chosen_input, output)).collect();
                for (o, chosen_output) in possible_outputs {
                    let mut new_possible = possible.clone();
                    let mut new_inputs = inputs.clone();
                    new_inputs.remove(0);
                    let mut new_outputs = outputs.clone();
                    new_outputs.remove(o);
                    if Self::map_segments(&mut new_possible, chosen_input, chosen_output) {
                        if let Some(solution) = Self::solve(new_possible, new_inputs.clone(), new_outputs) {
                            return Some(solution);
                        }
                    }
                }
                return None; // none of the available options worked
            }
        }
        let mut map = [0; 7];
        for m in 0..7 {
            map[m] = Self::definite(&possible[m]).expect("Map not complete");
        }
        Some(WireMap(map))
    }

    fn could_be(possible: &[[bool; 7]; 7], input: &Signal, output: &Signal) -> bool {
        if input.segments().len() != output.segments().len() {
            return false;
        }
        for i in input.segments() {
            let mut found = false;
            for o in output.segments() {
                if possible[i][o] {
                    found = true;
                    break;
                }
            }
            if !found {
                return false;
            }
        }
        true
    }

    fn definite(options: &[bool; 7]) -> Option<usize> {
        let mut single = None;
        for i in 0..7 {
            if options[i] {
                if single.is_none() {
                    single = Some(i);
                } else {
                    return None; // more than one option
                }
            }
        }
        if single.is_none() {
            panic!("No options available");
        }
        single
    }

    fn map_segments(possible: &mut [[bool; 7]; 7], input_signal: &Signal, output_signal: &Signal) -> bool {
        let mut inputs = input_signal.segments();
        let mut outputs = output_signal.segments();
        if inputs.len() != outputs.len() {
            panic!("Cannot map with different segment counts");
        }
        // remove any already mapped inputs and their respective output
        let mut i = 0;
        while i < inputs.len() {
            let input = inputs[i];
            if let Some(output) = Self::definite(&possible[input]) {
                if let Some(o) = outputs.iter().position(|o| *o == output) {
                    outputs.remove(o);
                    inputs.remove(i);
                } else {
                    return false; // invalid mapping
                }
            } else {
                i += 1;
            }
        }
        // mark each remaining input as only possibly mapping to any of the remaining outputs
        for input in inputs {
            for output in 0..7 {
                if outputs.iter().position(|o| *o == output).is_none() {
                    possible[input][output] = false;
                }
            }
        }
        true
    }
}