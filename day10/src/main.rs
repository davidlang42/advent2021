use std::env;
use std::fs;

#[derive(PartialEq, Copy, Clone, Debug)]
enum Bracket {
    Round,
    Square,
    Brace,
    Arrow
}

enum ParseResult {
    Valid,
    Incomplete { open: Vec<Bracket> },
    Corrupted { found: Bracket, expected: Option<Bracket> }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let mut syntax_score = 0;
        let mut auto_scores = Vec::new();
        for line in text.lines() {
            match parse_line(line) {
                ParseResult::Valid => {},
                ParseResult::Incomplete { open } => {
                    let mut score: u64 = 0;
                    for close in open.iter().rev() {
                        score *= 5;
                        score += match close {
                            Bracket::Round => 1,
                            Bracket::Square => 2,
                            Bracket::Brace => 3,
                            Bracket::Arrow => 4
                        };
                    }
                    auto_scores.push(score);
                    println!("{}: Incomplete score {}", line, score);
                },
                ParseResult::Corrupted { found, expected } => {
                    syntax_score += match found {
                        Bracket::Round => 3,
                        Bracket::Square => 57,
                        Bracket::Brace => 1197,
                        Bracket::Arrow => 25137
                    };
                    println!("{}: Expected {:?} but found {:?}", line, expected, found);
                }
            }
        }
        println!("Total syntax score: {}", syntax_score);
        auto_scores.sort();
        println!("Middle autocomplete score: {}", auto_scores[auto_scores.len()/2]);
    } else {
        println!("Please provide 1 argument: Filename");
    }
}

impl Bracket {
    fn open(c: char) -> Option<Self> {
        match c {
            '(' => Some(Self::Round),
            '[' => Some(Self::Square),
            '{' => Some(Self::Brace),
            '<' => Some(Self::Arrow),
            _ => None
        }
    }

    fn close(c: char) -> Option<Self> {
        match c {
            ')' => Some(Self::Round),
            ']' => Some(Self::Square),
            '}' => Some(Self::Brace),
            '>' => Some(Self::Arrow),
            _ => None
        }
    }
}

fn parse_line(line: &str) -> ParseResult {
    let mut brackets = Vec::new();
    let mut iter = line.chars();
    while let Some(c) = iter.next() {
        if let Some(close) = Bracket::close(c) {
            let last = brackets.pop();
            if last != Some(close) {
                return ParseResult::Corrupted {
                    found: close,
                    expected: last
                };
            }
        } else if let Some(open) = Bracket::open(c) {
            brackets.push(open);
        } else {
            panic!("Invalid char: {}", c);
        }
    }
    if brackets.len() == 0 {
        ParseResult::Valid
    } else {
        ParseResult::Incomplete { open: brackets }
    }
}