use std::env;
use std::fs;
use std::fmt::Display;
use std::fmt::Formatter;
use std::str::FromStr;

enum Number {
    Literal(usize),
    Pair(Box<Number>, Box<Number>)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let numbers: Vec<Number> = text.lines().map(|l| l.parse().unwrap()).collect();
        for number in numbers {
            println!("{}", number);
        }
    } else {
        println!("Please provide 1 argument: Filename");
    }
}

impl FromStr for Number {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        if line.len() > 2 && line[0..1] == *"[" && line[line.len()-1..line.len()] == *"]" {
            let inner = &line[1..(line.len()-1)];
            let comma = find_real_comma(&inner)?;
            Ok(Number::Pair(Box::new(inner[0..comma].parse()?), Box::new(inner[(comma+1)..].parse()?)))
        } else {
            Ok(Number::Literal(line.parse().map_err(|e| format!("{}: {}", e, line))?))
        }
    }
}

impl Display for Number {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        match self {
            Number::Literal(l) => write!(f, "{}", l),
            Number::Pair(a, b) => write!(f, "[{},{}]", a, b),
        }
    }
}

fn find_real_comma(line: &str) -> Result<usize, String> {
    let mut depth = 0;
    for (i, c) in line.chars().enumerate() {
        match c {
            '[' => depth += 1,
            ']' => depth -= 1,
            ',' if depth == 0 => return Ok(i),
            _ => {}
        }
    }
    Err(format!("No real comma found, final depth {}", depth))
}