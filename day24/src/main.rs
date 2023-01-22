use std::env;
use std::fs;
use std::str::FromStr;
use std::collections::HashMap;

#[derive(Eq, Hash, PartialEq, Copy, Clone)]
enum Variable {
    W,
    X,
    Y,
    Z
}

enum Operator {
    Add,
    Multiply,
    Divide,
    Modulo,
    Equal
}

enum Instruction {
    Input(Variable),
    VariableOperation(Variable, Operator, Variable),
    LiteralOperation(Variable, Operator, isize)
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
            (op, 2) => {
                let var1 = words[1].parse::<Variable>().map_err(|e| e.to_string())?;
                Ok(match words[2].parse::<Variable>() {
                    Ok(var2) => Self::VariableOperation(var1, op.parse()?, var2),
                    Err(_) => Self::LiteralOperation(var1, op.parse()?, words[2].parse::<isize>().map_err(|e| e.to_string())?)
                })
            },
            _ => Err(format!("Invalid instruction: {}", line))
        }
    }
}

// --------------------------------------------------------------------------------------------------------------

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let instructions: Vec<Instruction> = text.lines().map(|l| l.parse().unwrap()).collect();
        let mut alu = ArithmeticLogicUnit::new();
        for (i, instruction) in instructions.iter().enumerate() {
            println!("{}", i);
            alu.run(instruction);
        }
        println!("Z = {:?}", alu.get(&Variable::Z));
    } else {
        println!("Please provide 1 argument: Filename");
    }
}

// --------------------------------------------------------------------------------------------------------------

struct ArithmeticLogicUnit {
    variables: HashMap<Variable, Expression>,
    input_counter: usize
}

#[derive(Clone, Debug)]
enum Expression {
    Literal(isize),
    Input(usize),
    Sum(Vec<Expression>),
    Product(Vec<Expression>),
    Division(Box<Expression>, Box<Expression>),
    Modulo(Box<Expression>, Box<Expression>),
    Equal(Box<Expression>, Box<Expression>)
}

impl ArithmeticLogicUnit {
    fn new() -> Self {
        Self {
            variables: HashMap::new(),
            input_counter: 0
        }
    }

    fn get(&self, var: &Variable) -> Expression {
        if let Some(exp) = self.variables.get(var) {
            exp.clone()
        } else {
            Expression::Literal(0)
        }
    }

    fn set(&mut self, var: &Variable, exp: Expression) {
        self.variables.insert(*var, exp);
    }

    fn run(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Input(var) => {
                self.set(var, Expression::Input(self.input_counter));
                self.input_counter += 1;
            },
            Instruction::VariableOperation(var1, op, var2) => {
                let new_value = op.operate(self.get(var1), self.get(var2));
                self.set(var1, new_value);
            },
            Instruction::LiteralOperation(var1, op, literal) => {
                let new_value = op.operate(self.get(var1), Expression::Literal(*literal));
                self.set(var1, new_value);
            }
        }
    }
}

impl Operator {
    fn operate(&self, a: Expression, b: Expression) -> Expression {
        //TODO simplify
        match self {
            Self::Add => {
                let mut sum: Vec<Expression> = Vec::new();
                if let Expression::Sum(mut a_terms) = a {
                    sum.append(&mut a_terms);
                } else {
                    sum.push(a);
                }
                if let Expression::Sum(mut b_terms) = b {
                    sum.append(&mut b_terms);
                } else {
                    sum.push(b);
                }
                Expression::Sum(sum)
            },
            Self::Multiply => {
                let mut product: Vec<Expression> = Vec::new();
                if let Expression::Product(mut a_terms) = a {
                    product.append(&mut a_terms);
                } else {
                    product.push(a);
                }
                if let Expression::Product(mut b_terms) = b {
                    product.append(&mut b_terms);
                } else {
                    product.push(b);
                }
                Expression::Product(product)
            },
            Self::Divide => Expression::Division(Box::new(a), Box::new(b)),
            Self::Modulo => Expression::Modulo(Box::new(a), Box::new(b)),
            Self::Equal => Expression::Equal(Box::new(a), Box::new(b)),
        }
    }
}