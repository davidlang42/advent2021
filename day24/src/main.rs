use std::collections::HashSet;
use std::env;
use std::fs;
use std::hash::BuildHasher;
use std::collections::hash_map::DefaultHasher;
use std::str::FromStr;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

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
            Expression::from_literal(0)
        }
    }

    fn set(&mut self, var: &Variable, exp: Expression) {
        self.variables.insert(*var, exp);
    }

    fn run(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Input(var) => {
                if self.input_counter >= MAX_INPUTS {
                    panic!("Ran out of inputs");
                }
                self.set(var, Expression::from_input(self.input_counter));
                self.input_counter += 1;
            },
            Instruction::VariableOperation(var1, op, var2) => self.set(var1, Expression::from_operation(self.get(var1), op, self.get(var2))),
            Instruction::LiteralOperation(var1, op, literal) => self.set(var1, Expression::from_operation(self.get(var1), op, Expression::from_literal(*literal)))
        }
    }
}

// --------------------------------------------------------------------------------------------------------------

const MAX_INPUTS: usize = 14;

impl Hash for NormalExpression {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.offset.hash(state);
        self.terms.len().hash(state);
        let mut terms_h = 0;
        for term in &self.terms {
            let mut hasher = DefaultHasher::new();
            term.hash(&mut hasher);
            terms_h ^= hasher.finish();
        }
        state.write_u64(terms_h);
    }
}

impl Hash for ProductExpression {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.len().hash(state);
        let mut terms_h = 0;
        for term in &self.0 {
            let mut hasher = DefaultHasher::new();
            term.hash(&mut hasher);
            terms_h ^= hasher.finish();
        }
        state.write_u64(terms_h);
    }
}

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
enum Expression {
    Input(InputExpression),
    Normal(NormalExpression),
    Product(ProductExpression),
    Division(Box<Expression>, Box<Expression>),
    Modulo(Box<Expression>, Box<Expression>),
    Equal(Box<Expression>, Box<Expression>)
}

#[derive(Eq, PartialEq, Clone, Debug)]
struct NormalExpression {
    offset: isize,
    terms: HashMap<Expression, isize>
}

#[derive(Eq, PartialEq, Clone, Debug)]
struct ProductExpression(HashSet<Expression>);

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
struct InputExpression {
    powers: [usize; MAX_INPUTS]
}

impl Expression {
    fn from_literal(literal: isize) -> Self {
        Self::Normal(NormalExpression {
            offset: literal,
            terms: HashMap::new()
        })
    }

    fn from_input(index: usize) -> Self {
        let mut powers = [0; MAX_INPUTS];
        powers[index] = 1;
        Self::Input(InputExpression { powers })
    }

    fn from_operation(a: Expression, operator: &Operator, b: Expression) -> Expression {
        //TODO simplify
        match operator {
            Operator::Add => {
                if let Expression::Normal(mut normal_a) = a {
                    normal_a.add(b);
                    Expression::Normal(normal_a)
                } else if let Expression::Normal(mut normal_b) = b {
                    normal_b.add(a);
                    Expression::Normal(normal_b)
                } else {
                    let mut normal = NormalExpression::from_expression(a);
                    normal.add(b);
                    Expression::Normal(normal)
                }
            },
            Operator::Multiply => {
                if let Expression::Normal(mut normal_a) = a {
                    normal_a.multiply(b);
                    Expression::Normal(normal_a)
                } else if let Expression::Normal(mut normal_b) = b {
                    normal_b.multiply(a);
                    Expression::Normal(normal_b)
                } else {
                    let mut normal = NormalExpression::from_expression(a);
                    normal.multiply(b);
                    Expression::Normal(normal)
                }
            },
            Operator::Divide => Expression::Division(Box::new(a), Box::new(b)),
            Operator::Modulo => Expression::Modulo(Box::new(a), Box::new(b)),
            Operator::Equal => Expression::Equal(Box::new(a), Box::new(b)),
        }
    }
}

impl NormalExpression {
    fn from_expression(exp: Expression) -> Self {
        let mut terms = HashMap::new();
        terms.insert(exp, 1);
        Self {
            offset: 0,
            terms
        }
    }

    fn add(&mut self, exp: Expression) {
        if let Expression::Normal(normal) = exp {
            self.offset += normal.offset;
            for (term, factor) in normal.terms {
                self.add_term(term, factor);
            }
        } else {
            self.add_term(exp, 1);
        }
    }

    fn add_term(&mut self, term: Expression, factor: isize) {
        let new_factor = self.terms.get(&term).unwrap_or(&0) + factor;
        self.terms.insert(term, new_factor);
    }

    fn multiply(&mut self, exp: Expression) {
        if let Expression::Normal(normal) = exp {
            let a_offset = self.offset;
            let a_terms = self.terms.clone();
            let b_offset = normal.offset;
            let b_terms = normal.terms;
            self.offset = a_offset * b_offset;
            self.terms = HashMap::new();
            for (a_term, a_factor) in &a_terms {
                for (b_term, b_factor) in &b_terms {
                    self.add_term(Self::product_of(a_term.clone(), b_term.clone()), a_factor * b_factor);
                }
            }
            for (term, factor) in a_terms {
                self.add_term(term, factor * b_offset);
            }
            for (term, factor) in b_terms {
                self.add_term(term, factor * a_offset);
            }
        } else {
            let old_offset = self.offset;
            let old_terms = self.terms.clone();
            self.offset = 0;
            self.terms = HashMap::new();
            for (term, factor) in old_terms {
                self.add_term(Self::product_of(term, exp.clone()), factor);
            }
            self.add_term(exp, old_offset);
        }
    }

    fn product_of(a: Expression, b: Expression) -> Expression {
        //TODO simplify for input expressions
        let mut set = HashSet::new();
        set.insert(a);
        set.insert(b);
        Expression::Product(ProductExpression(set))
    }
}