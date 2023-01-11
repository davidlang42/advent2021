use std::env;
use std::fs;
use std::fmt::Display;
use std::fmt::Formatter;
use std::str::FromStr;

enum Number {
    Literal(usize),
    Pair(Box<Number>, Box<Number>)
}

struct Explosion {
    left: Option<usize>,
    right: Option<usize>
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let numbers: Vec<Number> = text.lines().map(|l| l.parse().unwrap()).collect();
        let mut result = Number::add(numbers[0].clone(), numbers[1].clone());
        for i in 2..numbers.len() {
            result = Number::add(result, numbers[i].clone());
        }
        println!("Result: {}", result);
        println!("Magnitude: {}", result.magnitude());
        let mut max_magnitude = 0;
        for x in 0..numbers.len() {
            for y in 0..numbers.len() {
                if x != y {
                    let result = Number::add(numbers[x].clone(), numbers[y].clone());
                    let magnitude = result.magnitude();
                    if magnitude > max_magnitude {
                        max_magnitude = magnitude;
                    }
                }
            }
        }
        println!("Best magnitude of 2 sum: {}", max_magnitude);
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

impl Number {
    fn add(a: Number, b: Number) -> Number {
        let mut n = Number::Pair(Box::new(a), Box::new(b));
        while n.reduce() {}
        n
    }

    fn reduce(&mut self) -> bool {
        self.explode_nested_pair(4).is_some() || self.split_literal(10)
    }

    fn explode_nested_pair(&mut self, at_depth: usize) -> Option<Explosion> {
        match self {
            Number::Literal(_) => None,
            Number::Pair(a, b) if at_depth == 0 => {
                // explode
                let a_value = a.as_literal().unwrap();
                let b_value = b.as_literal().unwrap();
                *self = Number::Literal(0);
                Some(Explosion {
                    left: Some(a_value),
                    right: Some(b_value)
                })
            },
            Number::Pair(a, b) => {
                if let Some(explosion) = a.explode_nested_pair(at_depth - 1) {
                    Some(b.consume_right(explosion))
                } else if let Some(explosion) = b.explode_nested_pair(at_depth - 1) {
                    Some(a.consume_left(explosion))
                } else {
                    None
                }
            }
        }
    }

    fn consume_right(&mut self, explosion: Explosion) -> Explosion {
        if let Some(right) = explosion.right {
            match self {
                Number::Literal(l) => {
                    *l += right;
                    Explosion {
                        left: explosion.left,
                        right: None
                    }
                },
                Number::Pair(a, b) => {
                    b.consume_right(a.consume_right(explosion))
                }
            }
        } else {
            explosion
        }
    }

    fn consume_left(&mut self, explosion: Explosion) -> Explosion {
        if let Some(left) = explosion.left {
            match self {
                Number::Literal(l) => {
                    *l += left;
                    Explosion {
                        left: None,
                        right: explosion.right
                    }
                },
                Number::Pair(a, b) => {
                    a.consume_left(b.consume_left(explosion))
                }
            }
        } else {
            explosion
        }
    }

    fn split_literal(&mut self, min_value: usize) -> bool {
        match self {
            Number::Literal(l) if *l >= min_value => {
                // split
                *self = Number::Pair(Box::new(Number::Literal(*l/2)), Box::new(Number::Literal((*l+1)/2)));
                true
            },
            Number::Literal(_) => false,
            Number::Pair(a, b) => a.split_literal(min_value) || b.split_literal(min_value)
        }
    }

    fn as_literal(&self) -> Option<usize> {
        match self {
            Number::Literal(l) => Some(*l),
            Number::Pair(_, _) => None
        }
    }

    fn magnitude(&self) -> usize {
        match self {
            Number::Literal(l) => *l,
            Number::Pair(a, b) => 3 * a.magnitude() + 2 * b.magnitude()
        }
    }
}

impl Clone for Number
{
    fn clone(&self) -> Self {
        match self {
            Number::Literal(l) => Number::Literal(*l),
            Number::Pair(a, b) => Number::Pair(Box::new(*a.clone()), Box::new(*b.clone()))
        }
    }
}