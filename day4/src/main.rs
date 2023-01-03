use std::env;
use std::fs;
use std::str::FromStr;
use std::fmt::Display;
use std::fmt::Formatter;

struct Board(Vec<Vec<Number>>);

struct Line<'a>(Vec<&'a Number>);

struct Number {
    value: usize,
    marked: bool
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let mut sections = text.split("\r\n\r\n");
        let calls: Vec<usize> = sections.next().unwrap().split(",").map(|n| n.parse().unwrap()).collect();
        let mut boards: Vec<Board> = sections.map(|s| s.parse().unwrap()).collect();
        let mut first = None;
        let mut last = None;
        for call in calls {
            let mut i = 0;
            while i < boards.len() {
                let board = &mut boards[i];
                board.mark(call);
                if let Some(line) = board.complete() {
                    if first.is_none() {
                        first = Some((i, line.to_string(), call, board.unmarked()));
                    }
                    last = Some((i, line.to_string(), call, board.unmarked()));
                    boards.remove(i);
                } else {
                    i += 1;
                }
            }
        }
        print_result("FIRST", first.unwrap());
        print_result("LAST", last.unwrap());
    } else {
        println!("Please provide 1 argument: Filename");
    }
}

fn print_result(description: &str, r: (usize, String, usize, usize)) {
    println!("Board #{} wins {}, completing line {} with {}, leaving Σ{} unmarked with a score of: {}", r.0+1, description, r.1, r.2, r.3, r.2*r.3);
}

impl FromStr for Board {
    type Err = String;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let mut rows = Vec::new();
        for line in text.lines() {
            let mut row = Vec::new();
            for number in line.split(" ").filter(|n| !n.is_empty()) {
                row.push(number.parse().unwrap())
            }
            rows.push(row);
        }
        Ok(Board::new(rows))
    }
}

impl Board {
    fn new(raw: Vec<Vec<usize>>) -> Self {
        let mut numbers = Vec::new();
        for r in 0..raw.len() {
            let mut row = Vec::new();
            for c in 0..raw[0].len() {
                let number = Number::new(raw[r][c]);
                row.push(number);
            }
            numbers.push(row);
        }
        Board(numbers)
    }

    fn rows(&self) -> Vec<Line> {
        let mut rows = Vec::new();
        for r in 0..self.0.len() {
            let mut row = Vec::new();
            for c in 0..self.0[0].len() {
                row.push(&self.0[r][c]);
            }
            rows.push(Line(row))
        }
        rows
    }

    fn columns(&self) -> Vec<Line> {
        let mut columns = Vec::new();
        for c in 0..self.0[0].len() {
            let mut column = Vec::new();
            for r in 0..self.0.len() {
                column.push(&self.0[r][c]);
            }
            columns.push(Line(column))
        }
        columns
    }

    fn mark(&mut self, value: usize) {
        for row in self.0.iter_mut() {
            for number in row.iter_mut() {
                if number.value == value {
                    number.marked = true;
                }
            }
        }
    }

    fn complete(&self) -> Option<Line> {
        for row in self.rows() {
            if row.complete() {
                return Some(row);
            }
        }
        for column in self.columns() {
            if column.complete() {
                return Some(column);
            }
        }
        None
    }

    fn unmarked(&self) -> usize {
        self.0.iter().flatten().filter(|n| !n.marked).map(|n| n.value).sum()
    }
}

impl Number {
    fn new(value: usize) -> Self {
        Number {
            value,
            marked: false
        }
    }
}

impl Display for Number {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        if self.marked {
            write!(f, "[{}]", self.value)?;
        } else {
            write!(f, "{}", self.value)?;
        }
        Ok(())
    }
}

impl Line<'_> {
    fn complete(&self) -> bool {
        for n in &self.0 {
            if !n.marked {
                return false;
            }
        }
        true
    }
}

impl Display for Line<'_> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        for (i, n) in self.0.iter().enumerate() {
            if i != 0 {
                write!(f, ",")?;
            }
            write!(f, "{}", n)?;
        }
        Ok(())
    }
}