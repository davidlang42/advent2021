
use std::str::FromStr;

pub enum Operator {
    Add,
    Multiply,
    Divide,
    Modulo,
    Equal
}

#[derive(Eq, Hash, PartialEq, Clone)]
pub enum Variable {
    W,
    X,
    Y,
    Z
}

pub enum Expression {
    Variable(Variable),
    Literal(isize)
}

pub enum Instruction {
    Input(Variable),
    Operation(Variable, Operator, Expression)
}

impl FromStr for Operator {
    type Err = String;

    fn from_str(word: &str) -> Result<Self, Self::Err> {
        match word {
            "add" => Ok(Self::Add),
            "mul" => Ok(Self::Multiply),
            "div" => Ok(Self::Divide),
            "mod" => Ok(Self::Modulo),
            "eql" => Ok(Self::Equal),
            _ => Err(format!("Invalid operator: {}", word))
        }
    }
}

impl FromStr for Variable {
    type Err = String;

    fn from_str(letter: &str) -> Result<Self, Self::Err> {
        match letter {
            "w" => Ok(Self::W),
            "x" => Ok(Self::X),
            "y" => Ok(Self::Y),
            "z" => Ok(Self::Z),
            _ => Err(format!("Invalid variable: {}", letter))
        }
    }
}

impl FromStr for Expression {
    type Err = String;

    fn from_str(expression: &str) -> Result<Self, Self::Err> {
        Ok(match expression.parse::<Variable>() {
            Ok(var) => Expression::Variable(var),
            Err(_) => Expression::Literal(expression.parse::<isize>().map_err(|e| e.to_string())?)
        })
    }
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let words: Vec<&str> = line.split(" ").collect();
        match (words[0], words.len() - 1) {
            ("inp", 1) => Ok(Self::Input(words[1].parse::<Variable>().map_err(|e| e.to_string())?)),
            (op, 2) => Ok(Self::Operation(
                words[1].parse::<Variable>().map_err(|e| e.to_string())?,
                op.parse()?,
                words[2].parse::<Expression>().map_err(|e| e.to_string())?,
            )),
            _ => Err(format!("Invalid instruction: {}", line))
        }
    }
}

impl Operator {
    pub fn operate(&self, a: isize, b: isize) -> isize {
        match self {
            Self::Add => a + b,
            Self::Multiply => a * b,
            Self::Divide => {
                if b == 0 {
                    panic!("b cannot be zero in a div operation");
                }
                a / b
            },
            Self::Modulo => {
                if a < 0 {
                    panic!("a cannot be less than zero in a mod operation");
                }
                if b <= 0 {
                    panic!("b cannot be less than or equal to zero in a mod operation");
                }
                a % b // negative numbers are disallowed
            },
            Self::Equal => if a == b { 1 } else { 0 }
        }
    }
}
