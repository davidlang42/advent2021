use std::env;
use std::fs;
use std::str::FromStr;
use std::collections::VecDeque;
use std::collections::HashMap;

struct ArithmeticLogicUnit {
    variables: HashMap<Variable, isize>,
    inputs: VecDeque<isize>
}

#[derive(Eq, Hash, PartialEq, Clone)]
enum Variable {
    W,
    X,
    Y,
    Z
}

enum Instruction {
    Input(Variable),
    Operation(Variable, Operator, Expression)
}

enum Expression {
    Variable(Variable),
    Literal(isize)
}

enum Operator {
    Add,
    Multiply,
    Divide,
    Modulo,
    Equal
}

impl Operator {
    fn operate(&self, a: isize, b: isize) -> isize {
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

impl ArithmeticLogicUnit {
    fn new(inputs: VecDeque<isize>) -> Self {
        Self {
            variables: HashMap::new(),
            inputs
        }
    }

    fn get(&self, var: &Variable) -> isize {
        *self.variables.get(var).unwrap_or(&0)
    }

    fn set(&mut self, var: Variable, value: isize) {
        self.variables.insert(var, value);
    }

    fn run(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Input(var) => {
                let new_value = self.inputs.pop_front().expect("Ran out of input values");
                self.set(var.clone(), new_value);
            },
            Instruction::Operation(var, op, exp) => {
                let a = self.get(var);
                let b = match exp {
                    Expression::Variable(b_var) => self.get(b_var),
                    Expression::Literal(b_literal) => *b_literal
                };
                let new_value = op.operate(a, b);
                self.set(var.clone(), new_value);
            }
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

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let instructions: Vec<Instruction> = text.lines().map(|l| l.parse().unwrap()).collect();
        let test_input = "13579246899999";
        let inputs: VecDeque<isize> = test_input.chars().map(|c| c.to_digit(10).unwrap() as isize).collect();
        let mut alu = ArithmeticLogicUnit::new(inputs);
        for instruction in &instructions {
            alu.run(instruction);
        }
        let valid = alu.get(&Variable::Z) == 0;
        println!("{} is {} model number", test_input, if valid { "a VALID" } else { "an INVALID "});
    } else {
        println!("Please provide 1 argument: Filename");
    }
}
