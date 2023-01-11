use std::env;
use std::fs;
use std::fmt::Display;
use std::fmt::Formatter;
use std::str::FromStr;

struct Pair(Number, Number);

enum Number {
    Literal(usize),
    Pair(Box<Pair>)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let pairs: Vec<Pair> = text.lines().map(|l| l.parse().unwrap()).collect();
        for pair in pairs {
            println!("{}", pair);
        }
    } else {
        println!("Please provide 1 argument: Filename");
    }
}

impl FromStr for Pair {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        if line.len() > 2 && line[0..1] == *"[" && line[line.len()-1..line.len()] == *"]" {
            let inner = &line[1..(line.len()-1)];
            let comma = find_real_comma(&inner)?;
            Ok(Pair(inner[0..comma].parse()?, inner[(comma+1)..].parse()?))
        } else {
            Err(format!("Not a pair: {}", line))
        }
    }
}

impl FromStr for Number {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        if let Ok(literal) = usize::from_str(line) {
            Ok(Number::Literal(literal))
        } else {
            Ok(Number::Pair(Box::new(line.parse()?)))
        }
    }
}

impl Display for Pair {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "[{},{}]", self.0, self.1)
    }
}

impl Display for Number {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        match self {
            Number::Literal(literal) => write!(f, "{}", literal),
            Number::Pair(boxed_pair) => write!(f, "{}", boxed_pair),
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